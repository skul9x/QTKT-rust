use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::time::{Instant, Duration};
use crate::secure_storage;
use crate::AppState;
use std::sync::atomic::Ordering;

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiRequest {
    pub contents: Vec<Content>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    pub content: Content,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GeminiError {
    ApiKeyMissing,
    ApiError(String),
    NetworkError(String),
}

impl From<GeminiError> for String {
    fn from(err: GeminiError) -> Self {
        match err {
            GeminiError::ApiKeyMissing => "Chưa cấu hình API Key. Vui lòng vào phần Cài đặt.".to_string(),
            GeminiError::ApiError(e) => format!("Lỗi từ Gemini API: {}", e),
            GeminiError::NetworkError(e) => format!("Lỗi kết nối mạng: {}", e),
        }
    }
}

const MODEL_NAME: &str = "gemini-3-flash-preview";
const COOLDOWN_DURATION: Duration = Duration::from_secs(180); // 3 phút như bản Python

#[tauri::command]
pub async fn generate_qtkt(app: tauri::AppHandle, state: tauri::State<'_, AppState>, topic: String) -> Result<String, String> {
    let api_keys = secure_storage::get_api_key(app).map_err(|_| String::from(GeminiError::ApiKeyMissing))?;
    if api_keys.is_empty() {
        return Err(String::from(GeminiError::ApiKeyMissing));
    }

    let prompt = format!(
"Bạn là chuyên gia y khoa hàng đầu. Hãy viết \"Quy trình kỹ thuật lâm sàng\" cho kỹ thuật: \"{}\"

Vui lòng viết CHÍNH XÁC theo mẫu \"Phụ lục 01: Hướng dẫn Quy trình kỹ thuật lâm sàng\" (Ban hành kèm theo Quyết định số 3023/QĐ-BYT ngày 28 tháng 7 năm 2023 của Bộ trưởng Bộ Y tế).

Yêu cầu nội dung:
- Viết bằng tiếng Việt chuyên ngành y khoa, văn phong chuyên nghiệp, súc tích.
- Nội dung phải chính xác, an toàn, cập nhật theo y văn mới nhất.
- Đầy đủ các mục lớn nhỏ theo đúng cấu trúc dưới đây.
- Các mục con (ví dụ 5.1, 5.2...) phải viết chi tiết, TUYỆT ĐỐI KHÔNG ĐƯỢC BỎ SÓT.

QUAN TRỌNG:
- KHÔNG được có lời chào (Chào bạn...), lời dẫn nhập, hay lời kết.
- KHÔNG được ký tên (Người viết, Chuyên gia...).
- TUYỆT ĐỐI KHÔNG lặp lại dòng chứa tên kỹ thuật ở đầu phần trả về (vì tôi đã có tiêu đề file riêng).
- Bắt đầu ngay bằng dòng \"1. ĐẠI CƯƠNG\" và kết thúc ở mục lục tham khảo.

CẤU TRÚC TRẢ VỀ:

1. ĐẠI CƯƠNG
(Định nghĩa, nguyên lý, mục đích của kỹ thuật)

2. CHỈ ĐỊNH
(Liệt kê các trường hợp bệnh lý hoặc tình huống lâm sàng được chỉ định thực hiện kỹ thuật)

3. CHỐNG CHỈ ĐỊNH
(Liệt kê các trường hợp tuyệt đối hoặc tương đối không được thực hiện)

4. THẬN TRỌNG
(Các trường hợp cần đặc biệt lưu ý hoặc thận trọng khi thực hiện)

5. CHUẨN BỊ
5.1. Người thực hiện
a) Nhân lực trực tiếp (Ví dụ: Bác sĩ chuyên khoa, Kỹ thuật viên, Điều dưỡng...)
b) Nhân lực hỗ trợ (Nếu có)

5.2. Thuốc
(Liệt kê cụ thể: Tên hoạt chất, nồng độ, hàm lượng, đường dùng, dạng dùng, sơ bộ số lượng nếu ước tính được)

5.3. Vật tư
(Vật tư tiêu hao dùng trực tiếp cho kỹ thuật: Bơm kim tiêm, găng tay, bông băng, gạc... - không bao gồm văn phòng phẩm)

5.4. Trang thiết bị
(Máy móc, dụng cụ y tế, phương tiện phục vụ kỹ thuật và hồi sức cấp cứu)

5.5. Người bệnh
- Giải thích cho người bệnh/người nhà về mục đích, tai biến có thể xảy ra.
- Chuẩn bị người bệnh: Tư thế, nhịn ăn (nếu cần), vệ sinh vùng can thiệp...

5.6. Hồ sơ bệnh án
(Các giấy tờ, cam kết, phiếu chỉ định cần thiết)

5.7. Thời gian thực hiện kỹ thuật (Ước tính khoảng bao nhiêu phút/giờ)

5.8. Địa điểm thực hiện kỹ thuật (Phòng thủ thuật, phòng mổ, hay tại giường...)

5.9. Kiểm tra hồ sơ
a) Kiểm tra người bệnh (Đúng người, đúng bệnh, đúng vị trí...)
b) Thực hiện bảng kiểm an toàn phẫu thuật/thủ thuật
c) Đặt tư thế người bệnh

6. TIẾN HÀNH QUY TRÌNH KỸ THUẬT
(Mô tả chi tiết từng bước thực hiện theo trình tự thời gian. Chia rõ ràng: 6.1. Bước 1; 6.2. Bước 2; ...)
LƯU Ý QUAN TRỌNG: Bước CUỐI CÙNG của mục 6 PHẢI là \"6.X. Kết thúc quy trình\" (X là số thứ tự bước cuối), với nội dung bắt buộc:
- Đánh giá tình trạng người bệnh sau thực hiện kỹ thuật.
- Hoàn thiện ghi chép hồ sơ bệnh án, lưu hồ sơ.
- Bàn giao người bệnh cho bộ phận tiếp theo (nếu có).

7. THEO DÕI VÀ XỬ TRÍ TAI BIẾN
7.1. Tai biến trong khi thực hiện kỹ thuật (Nhận biết và xử trí)
7.2. Tai biến sau khi thực hiện kỹ thuật (Nhận biết và xử trí)
7.3. Biến chứng muộn

8. TÀI LIỆU THAM KHẢO
(Liệt kê ít nhất 3 tài liệu tham khảo uy tín: Sách giáo khoa, Hướng dẫn của Bộ Y tế, Guidelines quốc tế...)",
        topic
    );

    let client = Client::new();
    let mut errors_log = Vec::new();

    // Duyệt qua danh sách keys
    for _ in 0..api_keys.len() {
        let current_idx = state.key_index.fetch_add(1, Ordering::SeqCst) % api_keys.len();
        let api_key = &api_keys[current_idx];
        let masked_key = format!("...{}", &api_key[api_key.len().saturating_sub(4)..]);

        // Kiểm tra status của key
        {
            let statuses = state.key_statuses.lock().unwrap();
            if let Some(status) = statuses.get(api_key) {
                if status.exhausted {
                    println!("  ⏭️ Bỏ qua Key {} ({}) - Đã hết quota", current_idx + 1, masked_key);
                    continue;
                }
                if let Some(until) = status.cooldown_until {
                    if Instant::now() < until {
                        println!("  ⏳ Bỏ qua Key {} ({}) - Cooldown...", current_idx + 1, masked_key);
                        continue;
                    }
                }
            }
        }

        println!("🚀 Thử Key {}/{} (Model: {})", current_idx + 1, api_keys.len(), MODEL_NAME);

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            MODEL_NAME, api_key
        );

        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt.clone() }],
            }],
        };

        let response_res = client
            .post(&url)
            .json(&request)
            .send()
            .await;

        match response_res {
            Ok(response) => {
                if response.status().is_success() {
                    let gemini_res: GeminiResponse = response
                        .json()
                        .await
                        .map_err(|e| String::from(GeminiError::ApiError(e.to_string())))?;

                    let result_text = gemini_res
                        .candidates
                        .get(0)
                        .and_then(|c| c.content.parts.get(0))
                        .map(|p| p.text.clone())
                        .ok_or_else(|| String::from(GeminiError::ApiError("Gemini returned an empty response".to_string())))?;

                    println!("✅ THÀNH CÔNG! (Model: {} | Key: {})", MODEL_NAME, masked_key);
                    return Ok(result_text);
                } else {
                    let status_code = response.status().as_u16();
                    let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                    println!("  ⚠️ Key {} lỗi: {} - {}", masked_key, status_code, error_text);

                    let mut statuses = state.key_statuses.lock().unwrap();
                    let status = statuses.entry(api_key.clone()).or_insert(crate::KeyStatus {
                        exhausted: false,
                        cooldown_until: None,
                    });

                    if status_code == 429 {
                        status.exhausted = true;
                        println!("  ⛔ Key {}: HẾT QUOTA - Bỏ qua vĩnh viễn!", masked_key);
                    } else if status_code == 503 {
                        status.cooldown_until = Some(Instant::now() + COOLDOWN_DURATION);
                        println!("  ⏸️ Key {}: SERVER BẬN - Cooldown 3 phút!", masked_key);
                    }
                    errors_log.push(format!("Key {}: {}", masked_key, error_text));
                }
            },
            Err(e) => {
                println!("  ❌ Lỗi mạng với Key {}: {}", masked_key, e);
                errors_log.push(format!("Key {}: Lỗi mạng", masked_key));
            }
        }
    }

    Err(format!(
        "❌ ĐÃ HẾT QUOTA HOẶC LỖI TẤT CẢ KEYS!\n\nChi tiết:\n{}",
        errors_log.join("\n")
    ))
}
