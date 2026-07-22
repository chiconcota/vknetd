# VKNETD SELF-IMPROVEMENT LOG & RULES

## 📌 Nguyên tắc Tự sửa lỗi (Self-Correction Rules)

### 1. Luôn kiểm tra log trước khi đoán lỗi
- **Lỗi:** Đoán mò nguyên nhân bug khi chưa thu thập full error traceback.
- **Quy tắc:** Bắt buộc thu thập log stdout/stderr hoặc `journalctl` trước khi sửa code.

### 2. Không làm rác thư mục dự án
- **Lỗi:** Tạo file `.md` hoặc file tạm ở gốc dự án.
- **Quy tắc:** Chỉ ghi tài liệu vào đúng 4 ngăn kéo trong `.vknetd-ai/`.

### 3. Đảm bảo an toàn Panic / Freeze Bàn phím
- **Lỗi:** Quên giải phóng `EVIOCGRAB` khi code bị panic.
- **Quy tắc:** Mọi thao tác grabbing bàn phím đều phải có Catch Panic / Signal Handler hoặc Watchdog bọc ngoài.

---

## 📜 Lịch sử Sửa đổi Hành vi
- **2026-07-23:** Đã thiết lập các quy tắc tự cải thiện ban đầu cho dự án `vknetd`.
