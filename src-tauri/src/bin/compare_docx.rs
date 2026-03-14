use docx_rs::*;
use std::fs::File;
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

fn main() {
    let title = "QUY TRÌNH KỸ THUẬT TEST CHUẨN";
    let content = "1. ĐẠI CƯƠNG\nĐây là nội dung đại cương để test in đậm.\n2. CHỈ ĐỊNH\n- Chỉ định 1\n- Chỉ định 2\n6. TIẾN HÀNH QUY TRÌNH KỸ THUẬT\n6.1. Bước 1: Chuẩn bị\nNội dung bước 1.\n6.2. Bước 2: Thực hiện\nNội dung bước 2.\nKết thúc quy trình";

    let mut docx = Docx::new()
        .page_margin(
            PageMargin::new()
                .top(1417)
                .bottom(1134)
                .left(1701)
                .right(1417),
        )
        .default_fonts(RunFonts::new().ascii("Times New Roman"))
        .default_size(26);

    docx = docx.add_paragraph(
        Paragraph::new()
            .add_run(Run::new().add_text(title.to_uppercase()).bold().size(28))
            .align(AlignmentType::Center)
            .line_spacing(LineSpacing::new().after(240)),
    );

    for line in content.lines() {
        let original_line = line.trim();
        if original_line.is_empty() {
            continue;
        }

        let is_bold_header = BOLD_PATTERNS.is_match(original_line);
        let is_generic_heading = GENERIC_HEADING.is_match(original_line);
        
        let cleaned_line = CLEAN_MD_HEADER.replace(original_line, "");
        let cleaned_line = CLEAN_MD_BOLD.replace_all(&cleaned_line, "$1");
        let cleaned_line = CLEAN_MD_ITALIC.replace_all(&cleaned_line, "$1");

        let mut run = Run::new().add_text(cleaned_line.to_string());
        if is_bold_header || is_generic_heading {
            println!("BOLD: {}", cleaned_line);
            run = run.bold();
        }

        docx = docx.add_paragraph(
            Paragraph::new()
                .add_run(run)
                .align(AlignmentType::Both)
                .line_spacing(LineSpacing::new().before(120).after(0)),
        );
    }

    let file = File::create("/tmp/rust_generated.docx").unwrap();
    docx.build().pack(file).unwrap();
    println!("Generated /tmp/rust_generated.docx");
}
