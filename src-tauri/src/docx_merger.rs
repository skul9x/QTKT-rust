use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use tauri::AppHandle;
use zip::ZipArchive;
use zip::write::ZipWriter;
use zip::write::SimpleFileOptions;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;

#[tauri::command]
pub async fn merge_docx_files(
    _app: AppHandle,
    paths: Vec<String>,
    output_name: String
) -> Result<String, String> {
    if paths.is_empty() {
        return Err("Danh sách file trống.".to_string());
    }

    // 1. Chuẩn bị đường dẫn đầu ra
    let merge_dir = crate::secure_storage::get_merge_export_path(_app.clone()).map_err(|e| format!("Lỗi thư mục: {}", e))?;
    let download_dir = Path::new(&merge_dir);
    let final_name = if output_name.to_lowercase().ends_with(".docx") {
        output_name
    } else {
        format!("{}.docx", output_name)
    };
    let output_path = download_dir.join(final_name);
    
    if !download_dir.exists() {
        std::fs::create_dir_all(&download_dir).map_err(|e| format!("Không thể tạo thư mục lưu file: {}", e))?;
    }

    if paths.len() == 1 {
        std::fs::copy(&paths[0], &output_path).map_err(|e| format!("Lỗi sao chép: {}", e))?;
        return Ok(output_path.to_string_lossy().to_string());
    }

    // 2. Logic gộp nâng cao: Thao tác XML trực tiếp để giữ định dạng
    // Chúng ta sẽ lấy file đầu tiên làm gốc, sau đó chèn nội dung các file sau vào body của nó.
    
    // Copy file đầu tiên làm file cơ sở
    std::fs::copy(&paths[0], &output_path).map_err(|e| format!("Lỗi tạo file cơ sở: {}", e))?;

    let mut merged_body_content = Vec::new();

    for (i, path_str) in paths.iter().enumerate() {
        let file = File::open(path_str).map_err(|e| format!("Không thể mở file {}: {}", path_str, e))?;
        let mut archive = ZipArchive::new(file).map_err(|e| format!("Lỗi đọc ZIP {}: {}", path_str, e))?;

        let mut doc_xml = archive.by_name("word/document.xml").map_err(|e| format!("Không thấy document.xml trong {}: {}", path_str, e))?;
        let mut content = String::new();
        doc_xml.read_to_string(&mut content).map_err(|e| format!("Lỗi đọc XML {}: {}", path_str, e))?;

        // Trích xuất nội dung bên trong <w:body>...<w:sectPr>
        let body_inner = extract_body_content(&content)?;
        
        if i > 0 {
            // Thêm page break
            merged_body_content.push("<w:p><w:r><w:br w:type=\"page\"/></w:r></w:p>".to_string());
        }
        merged_body_content.push(body_inner);
    }

    // Đọc file output và cập nhật document.xml
    update_docx_xml(&output_path, merged_body_content)?;

    Ok(output_path.to_string_lossy().to_string())
}

fn extract_body_content(xml: &str) -> Result<String, String> {
    let mut reader = Reader::from_str(xml);
    // CRITICAL: trim_text(false) to preserve spaces like <w:t xml:space="preserve"> </w:t>
    reader.config_mut().trim_text(false);

    let mut inside_body = false;
    let mut depth = 0;
    let mut buf = Vec::new();
    let mut result = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(event) => {
                match event {
                    Event::Start(ref e) => {
                        let name = e.name();
                        if name.as_ref() == b"w:body" {
                            inside_body = true;
                            depth = 0;
                        } else if inside_body {
                            depth += 1;
                            // Only skip w:sectPr if it's a direct child of w:body (depth == 1)
                            if name.as_ref() == b"w:sectPr" && depth == 1 {
                                // Skip
                            } else {
                                let mut writer = Writer::new(Vec::new());
                                writer.write_event(Event::Start(e.clone())).map_err(|e| format!("Writer error: {}", e))?;
                                result.push_str(&String::from_utf8_lossy(&writer.into_inner()));
                            }
                        }
                    }
                    Event::End(ref e) => {
                        let name = e.name();
                        if name.as_ref() == b"w:body" {
                            break;
                        } else if inside_body {
                            // Only skip w:sectPr if it's a direct child of w:body
                            if name.as_ref() == b"w:sectPr" && depth == 1 {
                                depth -= 1;
                            } else {
                                depth -= 1;
                                let mut writer = Writer::new(Vec::new());
                                writer.write_event(Event::End(e.clone())).map_err(|e| format!("Writer error: {}", e))?;
                                result.push_str(&String::from_utf8_lossy(&writer.into_inner()));
                            }
                        }
                    }
                    Event::Empty(ref e) => {
                        if inside_body {
                            let name = e.name();
                            // Only skip w:sectPr if it's a direct child of w:body
                            if name.as_ref() == b"w:sectPr" && depth == 0 {
                                // Skip
                            } else {
                                let mut writer = Writer::new(Vec::new());
                                writer.write_event(Event::Empty(e.clone())).map_err(|e| format!("Writer error: {}", e))?;
                                result.push_str(&String::from_utf8_lossy(&writer.into_inner()));
                            }
                        }
                    }
                    Event::Text(ref e) if inside_body => {
                        let mut writer = Writer::new(Vec::new());
                        writer.write_event(Event::Text(e.clone())).map_err(|e| format!("Writer error: {}", e))?;
                        result.push_str(&String::from_utf8_lossy(&writer.into_inner()));
                    }
                    Event::CData(ref e) if inside_body => {
                        let mut writer = Writer::new(Vec::new());
                        writer.write_event(Event::CData(e.clone())).map_err(|e| format!("Writer error: {}", e))?;
                        result.push_str(&String::from_utf8_lossy(&writer.into_inner()));
                    }
                    Event::Comment(ref e) if inside_body => {
                        let mut writer = Writer::new(Vec::new());
                        writer.write_event(Event::Comment(e.clone())).map_err(|e| format!("Writer error: {}", e))?;
                        result.push_str(&String::from_utf8_lossy(&writer.into_inner()));
                    }
                    Event::Eof => break,
                    _ => {}
                }
            }
            Err(e) => return Err(format!("XML Parse Error: {}", e)),
        }
        buf.clear();
    }

    Ok(result)
}

fn update_docx_xml(docx_path: &Path, body_parts: Vec<String>) -> Result<(), String> {
    // 1. Read entire file into memory
    let file = File::open(docx_path).map_err(|e| format!("File open error: {}", e))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("ZIP read error: {}", e))?;
    
    let mut entries = Vec::new();
    for i in 0..archive.len() {
        let entry = archive.by_index(i).map_err(|e| format!("Entry read error: {}", e))?;
        entries.push(entry.name().to_string());
    }

    // 2. Create temp file
    let temp_path = docx_path.with_extension("tmp");
    let temp_file = File::create(&temp_path).map_err(|e| format!("Temp file creation error: {}", e))?;
    let mut writer = ZipWriter::new(temp_file);

    for name in entries {
        let mut entry = archive.by_name(&name).map_err(|e| format!("Entry {} fetch error: {}", name, e))?;
        
        if name == "word/document.xml" {
            let mut content = String::new();
            entry.read_to_string(&mut content).map_err(|e| format!("XML read error: {}", e))?;
            
            let start_tag = "<w:body>";
            let end_tag = "</w:body>";
            let sect_tag = "<w:sectPr";
            
            let body_start = content.find(start_tag).ok_or("Could not find <w:body>")? + start_tag.len();
            let body_end = content.find(end_tag).ok_or("Could not find </w:body>")?;
            
            // CRITICAL: rfind (find from end) to get the LAST sectPr which belongs to the body
            let sect_start = content.rfind(sect_tag).ok_or("Could not find <w:sectPr> in base file")?;
            let sect_content = &content[sect_start..body_end];

            let mut new_document_xml = String::new();
            new_document_xml.push_str(&content[..body_start]);
            for part in &body_parts {
                new_document_xml.push_str(&part);
            }
            new_document_xml.push_str(sect_content);
            new_document_xml.push_str(end_tag);
            new_document_xml.push_str(&content[body_end + end_tag.len()..]);

            writer.start_file(name, SimpleFileOptions::default()).map_err(|e| format!("Start file error: {}", e))?;
            writer.write_all(new_document_xml.as_bytes()).map_err(|e| format!("Write XML error: {}", e))?;
        } else {
            let mut data = Vec::new();
            entry.read_to_end(&mut data).map_err(|e| format!("Read entry data error: {}", e))?;
            writer.start_file(name, SimpleFileOptions::default()).map_err(|e| format!("Start file error: {}", e))?;
            writer.write_all(&data).map_err(|e| format!("Write data error: {}", e))?;
        }
    }


    writer.finish().map_err(|e| format!("Lỗi đóng gói: {}", e))?;
    
    // 3. Thay thế file gốc bằng file tạm
    std::fs::rename(temp_path, docx_path).map_err(|e| format!("Lỗi ghi đè file: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_extraction_logic() {
        let xml = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
            <w:body>
                <w:p><w:t>Hello World</w:t></w:p>
                <w:p><w:t xml:space="preserve">  Perserve  </w:t></w:p>
                <w:sectPr><w:pgSz w:w="11906" w:h="16838"/></w:sectPr>
            </w:body>
        </w:document>"#;
        
        let extracted = extract_body_content(xml).unwrap();
        println!("DOCX_TEST_OUTPUT:START");
        println!("{}", extracted);
        println!("DOCX_TEST_OUTPUT:END");
        
        assert!(extracted.contains("Hello World"));
        assert!(extracted.contains("  Perserve  "));
        assert!(!extracted.contains("w:sectPr"));
    }
}
