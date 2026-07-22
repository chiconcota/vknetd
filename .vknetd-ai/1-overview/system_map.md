# VKNETD SYSTEM MAP & MODULE REGISTRY

## 📌 Trạng thái Dự án: 🟡 Phase 1 - Proof of Concept (Kernel Layer)

## 🎯 Mục tiêu Kiến trúc
Tạo bộ gõ tiếng Việt tự thân (`vknetd`) độc lập hoàn toàn, chạy ở tầng Kernel/Input Subsystem thông qua Linux `evdev` và `/dev/uinput`, đạt tốc độ $<1\text{ms}$, không preedit gạch chân, không phụ thuộc IBus hay Fcitx5.

## 🧩 Danh mục Module (Module Registry)

| Module | Đường dẫn / Thư mục | Trạng thái | Nhiệm vụ chính |
| :--- | :--- | :--- | :--- |
| **Kernel Layer** | `.vknetd-ai/3-modules/kernel-layer/` | 🟡 In Progress | Đánh chặn `evdev` (EVIOCGRAB), tạo bàn phím ảo `/dev/uinput`, Panic Safety & Watchdog |
| **Engine Layer** | `.vknetd-ai/3-modules/engine-layer/` | 🔴 Pending | Xử lý máy trạng thái Telex/VNI, Virtual Caret Index Buffer, Word Boundary rules |
| **System Layer** | `.vknetd-ai/3-modules/system-layer/` | 🔴 Pending | Udev rule `99-vknetd.rules`, Systemd User Service, Installer script |
| **UI & IPC Layer**| `.vknetd-ai/3-modules/ui-layer/` | 🔴 Pending | IPC Server (Unix Socket), System Tray Icon (VIE/ENG), GUI App |

---

## 📝 Recent Logs
- **2026-07-23:** Khởi tạo hệ thống bộ nhớ `.vknetd-ai` và quy trình `.agent`. Thống nhất kiến trúc cốt lõi (evdev EVIOCGRAB, uinput, udev rules, micro-delay 2ms chống lặp từ, Watchdog & Panic Recovery).
