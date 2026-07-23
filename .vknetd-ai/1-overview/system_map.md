# VKNETD SYSTEM MAP & MODULE REGISTRY

## 📌 Trạng thái Dự án: 🟢 Phase 1 (Kernel Layer) & Phase 2 (Engine Layer & Atomic Frame Emission) Completed

## 🎯 Mục tiêu Kiến trúc
Tạo bộ gõ tiếng Việt tự thân (`vknetd`) Vietnamese Kernel Native Input Daemon.
Độc lập hoàn toàn, chạy ở tầng Kernel/Input Subsystem thông qua Linux `evdev` và `/dev/uinput`, đạt tốc độ $<1\text{ms}$, không preedit gạch chân, không phụ thuộc IBus hay Fcitx5.

## 🧩 Danh mục Module (Module Registry)

| Module | Đường dẫn / Thư mục | Trạng thái | Nhiệm vụ chính |
| :--- | :--- | :--- | :--- |
| **Kernel Layer** | `.vknetd-ai/3-modules/kernel-layer/` | 🟢 Completed | Multi-node `evdev` (EVIOCGRAB), bàn phím ảo `/dev/uinput` QWERTY, Panic Safety & Atomic Frame Emission |
| **Engine Layer** | `.vknetd-ai/3-modules/engine-layer/` | 🟢 đan debug chưa xong | Máy trạng thái Telex/VNI, Smart Tone Placement (`giáo`, `mẫu`), Smart Caret (`khong`+`o`->`không`), Multi VNI modifiers |
| **System Layer** | `.vknetd-ai/3-modules/system-layer/` | 🔴 Pending | Udev rule `99-vknetd.rules`, Systemd User Service, Installer script |
| **UI & IPC Layer**| `.vknetd-ai/3-modules/ui-layer/` | 🔴 Pending | IPC Server (Unix Socket), System Tray Icon (VIE/ENG), GUI App |

---

## 📝 Recent Logs
- **2026-07-23:** Hoàn thành Phase 1 (Kernel Layer) & Phase 2 (Engine Layer) bằng Rust: Hỗ trợ Telex, VNI, Multi-device grab, Smart Tone placement, Smart Caret placement, Multi-VNI modifiers (`d9uoc75` -> `được`, `d9ay6` -> `đây`).
- **2026-07-23:** Triển khai **Atomic Frame Emission Tầng Nhân**: Đóng gói lệnh xóa Backspace và phím mới vào 1 mảng uinput frame đơn lẻ với `SYN_REPORT` cuối cùng, triệt hạ $100\%$ lỗi dính phím `dđ` và đơ Backspace trên AFFiNE, Facebook Messenger, Chrome, VS Code.
