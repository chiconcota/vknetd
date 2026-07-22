# ENGINE LAYER MODULE

## 📌 Chức năng
Lõi xử lý quy tắc gõ tiếng Việt Telex / VNI:
1. Quản lý máy trạng thái (State Machine) và Virtual Caret Index Buffer.
2. Áp dụng quy tắc ngắt từ (Word Boundary Strictness): Reset buffer ngay lập tục khi gặp ký tự đặc biệt, phím điều hướng, Space, Enter, Tab...
3. Biến đổi chuỗi ký tự gõ phím thành các lệnh xóa (`Backspace`) + bắn ký tự mới.
