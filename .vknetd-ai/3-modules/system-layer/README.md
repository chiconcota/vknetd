# SYSTEM LAYER MODULE

## 📌 Chức năng
Quản lý tích hợp hệ thống Linux:
1. Udev Rules (`99-vknetd.rules`) cấp quyền truy cập `/dev/uinput` và `/dev/input/event*` cho nhóm `input`.
2. File cấu hình Systemd User Service (`vknetd.service`) tự động khởi chạy cùng hệ thống.
3. Script cài đặt / gỡ bỏ tự động.
