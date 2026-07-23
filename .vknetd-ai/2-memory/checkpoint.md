# VKNETD CHECKPOINT (BÀN GIAO TIẾN ĐỘ & KẾ HOẠCH BƯỚC TIẾP THEO)

## 📌 Trạng thái Bàn giao hiện tại

- **Ngày bàn giao:** 2026-07-23
- **Nhánh Git làm việc:** `main`
- **Trạng thái:** Thống nhất Bản thiết kế Kiến trúc v2.0 - Phát phím Mã phần cứng Nguyên thủy Tầng Nhân (Kernel Hardware Keycode Emission).

---

## 🎯 Kết luận Phân tích Kỹ thuật & Thống nhất Định hướng (Session Achievements)
- [x] Phân tích video thực tế và log chi tiết trên Facebook Messenger, AFFiNE, Niri Launcher (`Super+R`).
- [x] Xác định nguyên nhân gốc rễ: Việc phụ thuộc vào chuỗi ISO 14755 (`Ctrl+Shift+U`) gây xung đột phím tắt `Ctrl+U`, khóa IM Context DOM của BlockSuite AFFiNE, và không chạy được trên Niri Launcher.
- [x] Thống nhất **Giải pháp Kiến trúc vknetd v2.0**:
  1. Phát phím bằng mã phím phần cứng Kernel (`KEY_ETH` cho `đ`, `KEY_COMPOSE`, XKB Latin-1 Mapping) trực tiếp qua `/dev/uinput`.
  2. Quy định mặc định hệ thống người dùng cài đặt bàn phím **English (US)** (UniKey / EVKey Standard).
  3. Lọc duy nhất Node Bàn phím chính QWERTY (`event7`), bỏ qua Node phụ Media/Fn (`event4`).
- [x] Đã khởi tạo Bản thiết kế chi tiết & Sơ đồ Luồng Mới (Mermaid) tại [`implementation_plan.md`](file:///home/chiconcota/.gemini/antigravity-ide/brain/2d66fdec-58d6-43ef-b7f3-941e542ffcc7/implementation_plan.md).
- [x] Đã cập nhật **Quyết định 011** trong [`decision-log.md`](file:///home/chiconcota/Documents/vnlilypadkey/.vknetd-ai/2-memory/decision-log.md).

---

## 🚀 Nhiệm vụ Triển khai Ngày mai (Tomorrow Implementation Plan)
- [ ] **Bước 1 (Primary Node Selector):** Cập nhật `src/keyboard/selector.rs` chỉ grab duy nhất Node Alpha QWERTY (`event7`).
- [ ] **Bước 2 (Virtual Hardware Layout Registration):** Cập nhật `src/keyboard/uinput_dev.rs` đăng ký bổ sung toàn bộ các bitmask mã phím phần cứng mở rộng (`KEY_ETH`, `KEY_COMPOSE`, Latin-1 keycodes).
- [ ] **Bước 3 (Hardware Keycode Mapping):** Cập nhật `src/engine/key_mapper.rs` xây dựng bảng tra cứu ánh xạ ký tự tiếng Việt sang mã phím phần cứng nguyên bản.
- [ ] **Bước 4 (Atomic Kernel Emission):** Cập nhật `src/main.rs` loại bỏ $100\%$ chuỗi `Ctrl+Shift+U`, phát gói mảng phím phần cứng nguyên thủy xuống Kernel.
- [ ] **Bước 5 (Verification):** Test thực tế trên Messenger, AFFiNE, Niri Launcher `Super+R`, Chrome, VS Code và chạy `cargo test`.

