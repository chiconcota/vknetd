# KERNEL LAYER MODULE (Status: 🟢 v1.0 Completed / v2.0 Architecture Approved)

## 📌 Chức năng
Quản lý giao tiếp trực tiếp với Linux Kernel Input Subsystem:
1. **Primary Keyboard Selector:** Đọc sự kiện bàn phím thô từ `/dev/input/event*` bằng `EVIOCGRAB` (`src/keyboard/selector.rs`), lọc duy nhất Node Alpha QWERTY chính.
2. **uinput Virtual Keyboard (Hardware Layout):** Tạo thiết bị bàn phím ảo bằng `/dev/uinput` (`src/keyboard/uinput_dev.rs`) với bảng mã phím mở rộng (`KEY_ETH`, `KEY_COMPOSE`, Latin-1 keycodes).
3. **Atomic Frame Emission:** Đóng gói mảng sự kiện `[Backspace x N + Hardware Keycodes + SYN_REPORT]` và phát trực tiếp xuống Kernel trong 1 system call `virt_device.emit(&batch)`.
4. **Panic Safety & Signal Catching:** Catch Panic Hook + Signal Listener (`SIGINT`, `SIGTERM`) + Emergency Hotkey (`LShift+RShift+Esc`) + Safety Auto-Timeout `--timeout` (`src/safety/panic_guard.rs`).

## 🚀 Cách chạy thử nghiệm
```bash
./target/release/vknetd --mode vni
```
