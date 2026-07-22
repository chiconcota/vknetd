# VKNETD CHECKPOINT (BÀN GIAO TIẾN ĐỘ)

## 📌 Trạng thái Bàn giao hiện tại

- **Ngày bàn giao:** 2026-07-23
- **Nhánh Git làm việc:** `main` (Khởi tạo dự án)
- **Phase hiện tại:** Phase 1 - Proof of Concept (Kernel Layer)

---

## 🎯 Công việc đã hoàn thành
- [x] Thống nhất bản thiết kế kiến trúc kỹ thuật bộ gõ `vknetd` (evdev EVIOCGRAB, uinput, state machine, word boundary rules, micro-delay).
- [x] Khởi tạo Hệ thống Bộ nhớ `.vknetd-ai` và Hệ thống Quy tắc `.agent` dành riêng cho `vknetd`.

---

## 🚧 Công việc đang thực hiện / Tiếp theo (Next Action)
- [ ] Khởi tạo dự án Rust (`Cargo.toml`) hoặc C++ tại thư mục dự án `/home/chiconcota/Documents/vnlilypadkey`.
- [ ] Bắt đầu viết **Bước 1 (Kernel Layer PoC)**:
  - Tạo module mở thiết bị bàn phím vật lý `/dev/input/event*` với `EVIOCGRAB`.
  - Tạo thiết bị bàn phím ảo `/dev/uinput`.
  - Viết vòng lặp pass-through phím thô từ `evdev` sang `uinput`.

---

## 🐛 Bug / Vấn đề cần theo dõi
- Chưa có (Dự án mới khởi tạo).
