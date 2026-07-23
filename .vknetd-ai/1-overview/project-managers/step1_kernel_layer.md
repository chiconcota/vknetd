# PHASE 1: KERNEL LAYER PROOF OF CONCEPT (POC)

## 📌 Mục tiêu
Xây dựng module đánh chặn bàn phím vật lý ở cấp độ nhân Linux thông qua `evdev` và phát lại tín hiệu phím thô thông qua bàn phím ảo `/dev/uinput`.

## 📋 Danh sách Công việc (Tasks Roadmap)
- [x] **Task 1.1:** Khởi tạo dự án Rust với các dependency liên quan đến `evdev`, `uinput`, `clap`, `libc`.
- [x] **Task 1.2:** Lấy danh sách thiết bị bàn phím trong `/dev/input/event*` và mở kết nối với ioctl `EVIOCGRAB`.
- [x] **Task 1.3:** Tạo thiết bị bàn phím ảo `/dev/uinput` với đầy đủ keycode chuẩn.
- [x] **Task 1.4:** Thiết lập vòng lặp nhận phím từ `evdev` và forward sang `/dev/uinput` (Pass-through test với Safety Auto-Timeout).
- [x] **Task 1.5:** Thêm Signal Handler (`SIGINT`, `SIGTERM`), Panic Hook & Emergency Hotkey (`LShift+RShift+Esc`) để tự động nhả `EVIOCGRAB` khi thoát chương trình.
