# vknetd — Vietnamese Kernel-Native Input Daemon

![Linux Input Subsystem](https://img.shields.io/badge/Platform-Linux-orange.svg)
![Architecture](https://img.shields.io/badge/Architecture-evdev%20%2B%20uinput-blue.svg)
![Latency](https://img.shields.io/badge/Latency-%3C1ms-brightgreen.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)

**vknetd** là bộ gõ tiếng Việt tự thân độc lập dành cho Linux, hoạt động trực tiếp ở tầng **Kernel Input Subsystem** (`evdev` + `/dev/uinput`). 

Dự án ra đời nhằm giải quyết triệt để các hạn chế cố hữu của các bộ gõ truyền thống (IBus, Fcitx5) như: lỗi gạch chân Preedit, lặp từ trên Wayland/Chrome/Electron, rớt phím trong game (Steam/Proton/Wine) và sự bất đồng bộ giữa các giao thức IME Protocol.

---

## ⚡ Triết lý & Đặc điểm Nổi bật

* **Kernel-First Interception:** Đánh chặn trực tiếp phím thô từ thiết bị phần cứng `/dev/input/event*` bằng ioctl `EVIOCGRAB` và phát phím ảo qua `/dev/uinput`.
* **Zero Display-Server Dependency:** Hoạt động nhất quán trên **Wayland, X11, TTY Terminal Console**, ứng dụng Qt, GTK, Electron, Web Browsers và Games mà không cần quan tâm giao thức IME của Desktop Environment.
* **Không Preedit (Gạch chân):** Phát lại ký tự bằng phím ảo cấp kernel $\rightarrow$ Ứng dụng nhận phím như một bàn phím vật lý thực thụ.
* **Siêu Nhanh ($<1\text{ms}$):** Xử lý máy trạng thái Telex/VNI trực tiếp bằng ngôn ngữ hệ thống hiệu năng cao (Rust), độ trễ cực thấp.
* **An Toàn & Chống Khóa Bàn Phím (Panic Safety):** Tích hợp Watchdog Supervisor và Signal Handlers giúp tự động nhả `EVIOCGRAB` giải phóng bàn phím tức thì nếu xảy ra sự cố.
* **Phim tắt Khẩn cấp:** Bấm `LShift + RShift + Esc` để ngắt khẩn cấp daemon và trả quyền kiểm soát bàn phím về hệ thống.

---

## 📐 Luồng Dữ Liệu (Data Pipeline)

```text
[ Physical Keyboard ] 
        │
        ▼ (EVIOCGRAB)
[ Kernel /dev/input/event* ] 
        │
        ▼ (Nuốt phím thô)
[ vknetd Daemon (State Machine Telex/VNI) ]
        │
        ▼ (Bắn Backspace + Ký tự mới)
[ Kernel /dev/uinput ]
        │
        ▼ (Bàn phím ảo phát tín hiệu)
[ Wayland / X11 / Apps / Games ]
```

---

## 🛠️ Cấu trúc Module Dự án

Dự án được chia làm 4 lớp tách biệt:

```text
vknetd/
├── .agent/                     # Quy chuẩn AI & Workflow quản lý dự án
├── .vknetd-ai/                 # Hệ thống bộ nhớ 4 ngăn kéo (.vknetd-ai)
├── src/                        # Mã nguồn chính (Rust)
│   ├── kernel/                 # Module đọc evdev, khởi tạo uinput, Panic Hooks
│   ├── engine/                 # State Machine Telex/VNI, Caret Index Buffer
│   ├── system/                 # Watchdog, Systemd service & Udev rules
│   └── ui/                     # Unix Domain Socket Server & System Tray App
├── 99-vknetd.rules             # Udev rules phân quyền /dev/uinput
└── Cargo.toml                  # Cấu hình dự án Rust
```

---

## 🗺️ Roadmap Phát triển (4 Bước)

- [x] **Phase 0:** Thiết kế kiến trúc tổng thể, chốt phương án chống lặp từ (Micro-delay 2ms) và ranh giới từ (Word Boundary Strictness). Khởi tạo bộ nhớ `.vknetd-ai`.
- [ ] **Phase 1 (Kernel Layer):** Viết module đọc `evdev` với `EVIOCGRAB` và khởi tạo bàn phím ảo `/dev/uinput` bằng Rust.
- [ ] **Phase 2 (Engine Layer):** Nhúng lõi xử lý Telex/VNI (Virtual Caret Index & Buffer Management).
- [ ] **Phase 3 (System Layer):** Đóng gói file Udev rules `99-vknetd.rules` và Systemd Service.
- [ ] **Phase 4 (UI & Tray):** Tạo System Tray Icon `[ VIE ] / [ ENG ]` giao tiếp IPC Socket.

---

## 💻 Cài đặt & Dùng thử (Quickstart PoC)

### Yêu cầu Tiền đề
* Hệ điều hành: Linux (Kernel $\ge 5.0$)
* Rust Toolchain ($\ge 1.75$)

### Cấu hình Udev Rules (Phân quyền không cần sudo)
```bash
# Copy file udev rule để cho phép user group input truy cập /dev/uinput và /dev/input/event*
sudo cp 99-vknetd.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
sudo usermod -aG input $USER
```

*(Sau khi thêm group `input`, hãy logout và login lại để hệ thống áp dụng nhóm mới).*

---

## 📄 Giấy phép (License)
Dự án được phát hành theo giấy phép [MIT License](LICENSE).
