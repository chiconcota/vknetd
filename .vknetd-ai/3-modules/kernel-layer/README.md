# KERNEL LAYER MODULE

## 📌 Chức năng
Quản lý giao tiếp trực tiếp với Linux Kernel Input Subsystem:
1. Đọc sự kiện bàn phím thô từ `/dev/input/event*` bằng `EVIOCGRAB`.
2. Tạo thiết bị bàn phím ảo bằng `/dev/uinput`.
3. Phát các sự kiện phím mới hoặc phím pass-through về hệ thống với `SYN_REPORT` và micro-delay $1 \sim 2\text{ms}$.
4. Xử lý Signal Catching & Panic Hook giải phóng `EVIOCGRAB`.
