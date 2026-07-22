# ENGINE LAYER MODULE (PLUGGABLE MULTI-LANGUAGE)

## 📌 Chức năng & Kiến trúc Mở (Pluggable Architecture)

Lớp Engine Layer được thiết kế theo mô hình **Plugin Interface (`trait ImeEngine`)** cho phép mở rộng đa ngôn ngữ trong tương lai để cộng đồng Open-Source cùng tham gia đóng góp.

```rust
pub trait ImeEngine {
    fn process_key(&mut self, key_event: KeyEvent) -> EngineResult;
    fn reset_buffer(&mut self);
    fn get_mode(&self) -> EngineMode;
}
```

### Các Engine Hỗ trợ:

1. **`VietnameseEngine` (Default - Core):**
   - Máy trạng thái Telex / VNI.
   - Quản lý Virtual Caret Index Buffer.
   - Áp dụng quy tắc ngắt từ (Word Boundary Strictness).
   - Xuất lệnh xóa `Backspace` + bắn phím biến đổi Unicode.

2. **`JapaneseEngine` (Future Plugin):**
   - Romaji $\rightarrow$ Hiragana $\rightarrow$ Kanji conversion engine (nhúng `mozc-core` hoặc `anthy`).
   - Gửi tín hiệu qua IPC Socket tới `vknetd-ui` để hiển thị Popup Candidate Window chọn từ Kanji.
