# KẾ HOẠCH CHI TIẾT KIẾN TRÚC VKNETD v2.0
## Phát phím Mã Phần cứng Nguyên thủy Tầng Nhân (Kernel-Native Hardware Keycode Emission)

- **Ngày khởi tạo:** 2026-07-23
- **Trạng thái:** 🟢 Đã phê duyệt kiến trúc - Chuẩn bị triển khai
- **Thư mục tài liệu:** `.vknetd-ai/1-overview/project-managers/v2_kernel_hardware_keycode_plan.md`

---

## 📌 1. Tổng quan Mục tiêu Kiến trúc
Chuyển đổi toàn bộ cơ chế phát phím của `vknetd` từ mẹo User-space ISO 14755 (`Ctrl+Shift+U`) sang **Phát phím Mã phần cứng Nguyên thủy Tầng Nhân (Kernel Hardware Keycode Emission)** qua `/dev/uinput`.

Loại bỏ $100\%$ các xung đột phím tắt `Ctrl+U`, đơ phím Backspace, dính chữ `dđ`/`oông`, và tương thích tuyệt đối $100\%$ trên mọi ứng dụng Linux (AFFiNE, Facebook Messenger, Niri Launcher `Super+R`, Chrome, VS Code, Game Steam, Terminal).

---

## 🔄 2. Sơ đồ Luồng Hệ thống Mới (New System & User Flow)

```mermaid
flowchart TD
    subgraph HW ["1. Tầng Bàn phím Phần cứng (Hardware Input)"]
        USB_KB["Bàn phím USB (QWERTY Layout US)"]
        PRIMARY_NODE["/dev/input/event7 (Primary Alpha Node)"]
        MEDIA_NODE["/dev/input/event4 (Media/Fn Node - Skipped)"]
        
        USB_KB --> PRIMARY_NODE
        USB_KB -.->|Bỏ qua Grab| MEDIA_NODE
    end

    subgraph KL ["2. Tầng Kernel Layer (vknetd Grab & Safety)"]
        SELECTOR["Primary Node Selector<br/><i>Chỉ Grab duy nhất Node Alpha QWERTY</i>"]
        PANIC_GUARD["Safety Panic Guard<br/><i>LShift + RShift + Esc / Catch Signals</i>"]
        EVDEV_LOOP["Kernel Event Loop<br/><i>Đọc mã phím thô (Scancodes)</i>"]

        PRIMARY_NODE --> SELECTOR --> PANIC_GUARD --> EVDEV_LOOP
    end

    subgraph EL ["3. Tầng Engine Layer (Máy trạng thái Telex/VNI)"]
        PARSER{"Phân loại Phím"}
        BOUND_CHECK{"Word Boundary Key?<br/><i>Space / Enter / Mũi tên / Tab</i>"}
        RESET_STATE["Reset Caret Buffer"]
        STATE_ENGINE["Telex / VNI State Machine<br/><i>Xử lý âm tiết & Smart Tone placement</i>"]
        HW_MAPPER["Hardware Keycode Mapper<br/><i>Ánh xạ ký tự -> Mã phím phần cứng uinput</i>"]

        EVDEV_LOOP --> PARSER
        PARSER -->|Ký tự biên từ| BOUND_CHECK -->|Đúng| RESET_STATE --> PASSTHROUGH
        PARSER -->|Ký tự từ| STATE_ENGINE --> HW_MAPPER
    end

    subgraph EMIT ["4. Tầng Phát phím Ảo (Kernel Virtual Driver)"]
        PASSTHROUGH["PassThrough Single Event<br/><i>[Physical Key + SYN_REPORT]</i>"]
        ATOMIC_HW_FRAME["Atomic Hardware Frame<br/><i>[Backspace x N + Hardware Keycodes + SYN_REPORT]</i>"]
        VIRT_DEV["Bàn phím ảo /dev/uinput<br/><i>(QWERTY + Extended Hardware Keycodes)</i>"]

        HW_MAPPER --> ATOMIC_HW_FRAME --> VIRT_DEV
        PASSTHROUGH --> VIRT_DEV
    end

    subgraph OS ["5. Tầng Hệ điều hành & Màn hình"]
        LIBINPUT["libinput Subsystem"]
        WAYLAND["Display Server (Wayland Niri / Hyprland / X11)"]
        XKB["XKB Layout Engine (Standard US QWERTY)"]
        APPS["Ứng dụng Màn hình<br/><i>(AFFiNE / Messenger / Niri Launcher / Terminal)</i>"]

        VIRT_DEV --> LIBINPUT --> WAYLAND --> XKB --> APPS
    end

    style HW fill:#2b2d42,stroke:#8d99ae,color:#fff
    style KL fill:#1d3557,stroke:#457b9d,color:#fff
    style EL fill:#2a9d8f,stroke:#e9c46a,color:#fff
    style EMIT fill:#e76f51,stroke:#f4a261,color:#fff
    style OS fill:#3d5a80,stroke:#98c1d9,color:#fff
```

---

## 🎯 3. Chi tiết Công việc Triển khai (Step-by-Step Roadmap)

### 📌 Bước 1: Filter Bàn phím phần cứng (`src/keyboard/selector.rs`)
- [ ] Cấu hình chỉ grab độc quyền node bàn phím chính (Alpha QWERTY keys `event7`), bỏ qua node phụ Media/Fn (`event4`) để loại bỏ $100\%$ hiện tượng lặp phím bấm (Race Condition).

### 📌 Bước 2: Đăng ký Bàn phím Ảo Phần cứng Mở rộng (`src/keyboard/uinput_dev.rs`)
- [ ] Đăng ký bổ sung toàn bộ các mã phím phần cứng chuẩn Linux Kernel (`KEY_ETH` cho `đ`, `KEY_COMPOSE`, các mã phím Latin Extended) vào `VirtualDeviceBuilder` thông qua `UI_SET_KEYBIT`.

### 📌 Bước 3: Ánh xạ Ký tự sang Mã phím Phần cứng (`src/engine/key_mapper.rs` & `src/main.rs`)
- [ ] Xây dựng bảng ánh xạ trực tiếp `char_to_hardware_keycode(c: char) -> Option<Key>`.
- [ ] Gỡ bỏ hoàn toàn chuỗi phím rác `Ctrl+Shift+U` (ISO 14755) và các lệnh `sleep` trễ.

### 📌 Bước 4: Phát sự kiện Nguyên tử Nguyên bản (`src/main.rs`)
- [ ] Đóng gói mảng sự kiện `[Backspace x N + Hardware Keycodes + SYN_REPORT]` và phát trực tiếp xuống `/dev/uinput` trong đúng 1 lệnh `virt_device.emit(&batch)`.

---

## 🧪 4. Kế hoạch Kiểm thử & Xác minh (Verification Plan)

### Automated Tests
- [ ] Chạy `cargo test` kiểm tra 100% unit tests của Telex/VNI state machine.
- [ ] Kiểm tra tính đúng đắn của hàm ánh xạ mã phím phần cứng `char_to_hardware_keycode`.

### Manual Tests (Thực tế trên Môi trường Người dùng)
- [ ] **Facebook Messenger Web:** Kiểm tra gõ `d9` $\rightarrow$ `đ` (không dính `dđ`), `o6ng` $\rightarrow$ `ông` (không dính `oông`), gõ xóa phím Backspace mượt mà.
- [ ] **AFFiNE BlockSuite Editor:** Gõ `d9uoc75` $\rightarrow$ `được`, gõ xóa chữ dài `yuda` bằng phím Backspace vật lý nhạy $100\%$.
- [ ] **Niri Launcher (`Super + R`):** Bấm `Super + R`, gõ `d9ay` $\rightarrow$ `đây`, `sao1` $\rightarrow$ `sáo`.
- [ ] **Terminal & VS Code:** Kiểm tra gõ tiếng Việt tự nhiên nét căng.
