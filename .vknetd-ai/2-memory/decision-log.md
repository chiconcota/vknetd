# VKNETD DECISION LOG (NHẬT KÝ QUYẾT ĐỊNH KIẾN TRÚC)

## 📌 Quy định:
Nơi lưu trữ tất cả các Quyết Định Kỹ Thuật Lõi (Core Technical Decisions) đã được chốt và triển khai trong dự án.

---

### [2026-07-23] Quyết định 001: Đánh chặn Tầng Phần cứng bằng evdev & uinput
- **Bối cảnh:** Bộ gõ trên Linux (IBus, Fcitx5) hay bị lỗi gạch chân preedit, bất đồng bộ với Wayland text-input protocol, lỗi trên Chrome/Electron và các game chạy qua Steam/Wine.
- **Quyết định:** Sử dụng Linux Input Subsystem nguyên thủy:
  1. Dùng `evdev` với ioctl `EVIOCGRAB` để chiếm quyền đọc độc quyền phím thô từ `/dev/input/event*`.
  2. Dùng `/dev/uinput` để tạo bàn phím ảo phát phím biến đổi lại cho hệ thống.
- **Kết quả:** Đạt trễ $<1\text{ms}$, bỏ hoàn toàn preedit, không phụ thuộc Display Server (Wayland/X11).

---

### [2026-07-23] Quyết định 002: Quy tắc Ranh giới Từ (Word Boundary) & Chống Lệch Buffer
- **Bối cảnh:** Các ứng dụng IDE có tính năng Auto-closing pairs `()` `[]` `""` có thể khiến lệnh `Backspace` của bộ gõ xóa sai vị trí con trỏ.
- **Quyết định:** Đặt quy tắc ngắt từ cứng: Tất cả các ký tự không phải chữ cái Tiếng Việt/English (ngoặc, toán tử, phím mũi tên, Mouse click, Space, Enter, Tab, Esc) sẽ lập tức **XÓA RỖNG (RESET) BUFFER**.
- **Kết quả:** Loại bỏ $100\%$ xung đột với IDE Auto-complete / Auto-closing pairs.

---

### [2026-07-23] Quyết định 003: Giải quyết Race Condition & Lặp Từ bằng Micro-delay
- **Bối cảnh:** Khi phát lệnh `Backspace` và `Phím mới` quá nhanh (vài $\mu\text{s}$), các app GUI nặng (Electron, Chrome) bị giật lag và xử lý không kịp lệnh xóa, gây ra lỗi lặp từ (ví dụ `aa` thành `aâ`).
- **Quyết định:**
  1. Tách biệt gói ngắt `SYN_REPORT` giữa lệnh `Backspace` và `Phím mới`.
  2. Chèn khoảng trễ nhỏ $1 \sim 2\text{ms}$ (Micro-delay) giữa Backspace và Phím mới để UI Event Loop của App kịp xóa ký tự cũ.
- **Kết quả:** Đảm bảo xóa chữ chính xác tuyệt đối mà người dùng không thể nhận thấy độ trễ.

---

### [2026-07-23] Quyết định 004: Chạy ở User Space với Udev Rule & Phân quyền an toàn
- **Bối cảnh:** Chạy daemon bằng Udev rule tại User space để tiện cài đặt và quản lý, tránh phụ thuộc vào sudo root.
- **Quyết định:** Sử dụng Udev rule `99-vknetd.rules` cấp quyền `0660` nhóm `input` cho `/dev/uinput` và `/dev/input/event*`.
- **Kết quả:** Đảm bảo `vknetd` hoạt động ở tầng phần cứng `evdev` mà không cần quyền root khi chạy thông thường.
