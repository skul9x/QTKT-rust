use docx_rs::*;
use docx_rs::LineSpacing;
use std::fs::File;
use tauri::AppHandle;
use regex::{Regex, RegexSet};

lazy_static::lazy_static! {
    static ref BOLD_PATTERNS: RegexSet = RegexSet::new(&[
        r"(?i)^TÊN KỸ THUẬT[:;]?",
        r"(?i)^\d+\.\s+ĐẠI CƯƠNG",
        r"(?i)^\d+\.\s+CHỈ ĐỊNH",
        r"(?i)^\d+\.\s+CHỐNG CHỈ ĐỊNH",
        r"(?i)^\d+\.\s+THẬN TRỌNG",
        r"(?i)^\d+\.\s+CHUẨN BỊ",
        r"(?i)^\d+\.\d+\.\s+Người thực hiện",
        r"(?i)^\d+\.\d+\.\s+Thuốc",
        r"(?i)^\d+\.\d+\.\s+Vật tư",
        r"(?i)^\d+\.\d+\.\s+Trang thiết bị",
        r"(?i)^\d+\.\d+\.\s+Người bệnh",
        r"(?i)^\d+\.\d+\.\s+Hồ sơ bệnh án",
        r"(?i)^\d+\.\d+\.\s+Thời gian thực hiện kỹ thuật",
        r"(?i)^\d+\.\d+\.\s+Địa điểm thực hiện kỹ thuật",
        r"(?i)^\d+\.\d+\.\s+Kiểm tra hồ sơ",
        r"(?i)^\d+\.\s+TIẾN HÀNH QUY TRÌNH KỸ THUẬT",
        r"(?i)^\d+\.\d+\.\s+Bước\s+\d+",
        r"(?i)^\d+\.\s+THEO DÕI VÀ XỬ TRÍ TAI BIẾN",
        r"(?i)^TÀI LIỆU THAM KHẢO"
    ]).unwrap();

    static ref GENERIC_HEADING: Regex = Regex::new(r"(?i)^(\d+\.|\#{1,3})\s").unwrap();
    static ref CLEAN_MD_HEADER: Regex = Regex::new(r"^\#{1,3}\s*").unwrap();
    static ref CLEAN_MD_BOLD: Regex = Regex::new(r"\*\*([^*]+)\*\*").unwrap();
    static ref CLEAN_MD_ITALIC: Regex = Regex::new(r"\*([^*]+)\*").unwrap();
}

#[tauri::command]
pub async fn export_to_docx(app: AppHandle, title: String, content: String) -> Result<String, String> {
    // 1. Xác định đường dẫn lưu file
    let export_path = crate::secure_storage::get_export_path(app).map_err(|e| format!("{}", e))?;
    let download_dir = std::path::PathBuf::from(export_path);
    let file_name = format!("{}.docx", title.replace("/", "_").replace("\\", "_"));
    let file_path = download_dir.join(&file_name);

    // 2. Khởi tạo tài liệu Word với các thiết lập chuẩn QĐ 3023/QĐ-BYT
    let mut docx = Docx::new()
        .page_margin(
            PageMargin::new()
                .top(1417)    // 2.5 cm (1 cm = 567 twips)
                .bottom(1134) // 2.0 cm
                .left(1701)   // 3.0 cm
                .right(1417),  // 2.5 cm
        )
        .default_fonts(RunFonts::new().ascii("Times New Roman"))
        .default_size(26); // Size 13pt (1pt = 2 half-points)

    // 3. Tiêu đề quy trình (14pt, Bold, Center, Space After 12pt)
    docx = docx.add_paragraph(
        Paragraph::new()
            .add_run(Run::new().add_text(title.to_uppercase()).bold().size(28))
            .align(AlignmentType::Center)
            .line_spacing(LineSpacing::new().after(240)),
    );

    // 4. Xử lý nội dung (Regex matching logic từ Python)
    for line in content.lines() {
        let original_line = line.trim();
        if original_line.is_empty() {
            continue;
        }

        // Kiểm tra in đậm (Bold headers)
        let is_bold_header = BOLD_PATTERNS.is_match(original_line);
        let is_generic_heading = GENERIC_HEADING.is_match(original_line);
        
        // Clean markdown symbols
        let cleaned_line = CLEAN_MD_HEADER.replace(original_line, "");
        let cleaned_line = CLEAN_MD_BOLD.replace_all(&cleaned_line, "$1");
        let cleaned_line = CLEAN_MD_ITALIC.replace_all(&cleaned_line, "$1");

        let mut run = Run::new().add_text(cleaned_line.to_string());
        if is_bold_header || is_generic_heading {
            run = run.bold();
        }

        docx = docx.add_paragraph(
            Paragraph::new()
                .add_run(run)
                .align(AlignmentType::Both)
                .line_spacing(LineSpacing::new().before(120).after(0)), // Space Before 6pt, After 0pt
        );
    }

    // 5. Lưu file
    if !download_dir.exists() {
        std::fs::create_dir_all(&download_dir).map_err(|e| format!("Không thể tạo thư mục lưu file: {}", e))?;
    }
    let file = File::create(&file_path).map_err(|e| format!("Không thể tạo file: {}", e))?;
    docx.build().pack(file).map_err(|e| format!("Lỗi khi đóng gói file Word: {}", e))?;

    Ok(file_path.to_string_lossy().to_string())
}
