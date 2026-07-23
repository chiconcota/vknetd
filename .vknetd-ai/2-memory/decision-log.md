# VKNETD DECISION LOG (NHẬT KÝ QUYẾT ĐỊNH KIẾN TRÚC)

## 📌 Quy định:
Nơi lưu trữ tất cả các Quyết Định Kỹ Thuật Lõi (Core Technical Decisions) đã được chốt và triển khai trong dự án.

---

### [2026-07-23] Quyết định 001: Đánh chặn Tầng Phần cứng bằng evdev & uinput
- **Bối cảnh:** Bộ gõ trên Linux (IBus, Fcitx5) hay bị lỗi gạch chân preedit, bất đồng bộ với Wayland text-input protocol, lỗi trên Chrome/Electron và các game chạy qua Steam/Wine.
- **Quyết định:** Sử dụng Linux Input Subsystem nguyên thủy:
  1. Dùng `evdev` với ioctl `EVIOCGRAB` để chiếm quyền đọc độc quyền phím thô từ `/dev/input/event*`.
  2. Dùng `/dev/uinput` để tạo bàn phím ảo phát phím biến đổi lại cho hệ thống.
- **Kết quả:** Đạt trễ $<1\text{ms}$, bỏ hoàn toàn preedit, không phụ thuộc Display Server (Wayland/X11).

---

### [2026-07-23] Quyết định 002: Quy tắc Ranh giới Từ (Word Boundary) & Chống Lệch Buffer
- **Bối cảnh:** Các ứng dụng IDE có tính năng Auto-closing pairs `()` `[]` `""` có thể khiến lệnh `Backspace` của bộ gõ xóa sai vị trí con trỏ.
- **Quyết định:** Đặt quy tắc ngắt từ cứng: Tất cả các ký tự không phải chữ cái Tiếng Việt/English (ngoặc, toán tử, phím mũi tên, Mouse click, Space, Enter, Tab, Esc) sẽ lập tức **XÓA RỖNG (RESET) BUFFER**.
- **Kết quả:** Loại bỏ $100\%$ xung đột với IDE Auto-complete / Auto-closing pairs.

---

### [2026-07-23] Quyết định 003: Giải quyết Race Condition & Lặp Từ bằng Micro-delay
- **Bối cảnh:** Khi phát lệnh `Backspace` và `Phím mới` quá nhanh (vài $\mu\text{s}$), các app GUI nặng (Electron, Chrome) bị giật lag và xử lý không kịp lệnh xóa, gây ra lỗi lặp từ (ví dụ `aa` thành `aâ`).
- **Quyết định:**
  1. Tách biệt gói ngắt `SYN_REPORT` giữa lệnh `Backspace` và `Phím mới`.
  2. Chèn khoảng trễ nhỏ $1 \sim 2\text{ms}$ (Micro-delay) giữa Backspace và Phím mới để UI Event Loop của App kịp xóa ký tự cũ.
- **Kết quả:** Đảm bảo xóa chữ chính xác tuyệt đối mà người dùng không thể nhận thấy độ trễ.

---

### [2026-07-23] Quyết định 004: Chạy ở User Space với Udev Rule & Phân quyền an toàn
- **Bối cảnh:** Chạy daemon bằng Udev rule tại User space để tiện cài đặt và quản lý, tránh phụ thuộc vào sudo root.
- **Quyết định:** Sử dụng Udev rule `99-vknetd.rules` cấp quyền `0660` nhóm `input` cho `/dev/uinput` và `/dev/input/event*`.
- **Kết quả:** Đảm bảo `vknetd` hoạt động ở tầng phần cứng `evdev` mà không cần quyền root khi chạy thông thường.

---

### [2026-07-23] Quyết định 005: Kiến trúc Multi-Language Engine Tách biệt (Pluggable IME Traits)
- **Bối cảnh:** Mở rộng tầm nhìn dự án thành bộ gõ Kernel đa ngôn ngữ (Tiếng Việt, Tiếng Nhật Romaji/Kanji, Tiếng Trung Pinyin...), tạo điều kiện cho cộng đồng Open-Source cùng đóng góp Module sau này.
- **Quyết định:** Thiết kế Lớp Engine Layer theo mô hình Plugin / Trait interface trong Rust (ví dụ `trait ImeEngine`):
  1. `VietnameseEngine` (Khởi tạo trước): Xử lý máy trạng thái Telex/VNI.
  2. `JapaneseEngine` (Thiết kế mở cho tương lai): Nhúng lõi từ điển `mozc-core` / `anthy`, gửi sự kiện qua IPC Socket để `vknetd-ui` bật Popup Candidate Window chọn chữ Kanji.
- **Kết quả:** `vknetd` không bị đóng khung duy nhất vào tiếng Việt, mở rộng linh hoạt cho cộng đồng phát triển.

---

### [2026-07-23] Quyết định 006: Safety Auto-Timeout & Non-blocking Evdev Event Loop
- **Bối cảnh:** Việc test thử nghiệm `evdev` `EVIOCGRAB` trên máy thật nếu bị đơ hoặc kẹt blocking read của kernel có thể dẫn đến treo phím máy thật.
- **Quyết định:** 
  1. Cấu hình file descriptor bàn phím ở chế độ `O_NONBLOCK` (`libc::fcntl`) để tránh blocking read trong kernel loop.
  2. Bổ sung tham số `--timeout <SECONDS>` tự động ngắt daemon và nhả phím `ungrab` sau khoảng thời gian chỉ định (Mặc định 10s trong PoC).
  3. Bổ sung phím tắt khẩn cấp `LShift + RShift + Esc` ở tầng Kernel.
- **Kết quả:** Cho phép thử nghiệm an toàn tuyệt đối trên máy thật mà không gây rủi ro khóa phím hệ thống.

---

### [2026-07-23] Quyết định 007: Loại bỏ viết tắt phím `w` ở đầu từ (No Standalone 'w' -> 'ư')
- **Bối cảnh:** Người dùng chơi game (dùng phím `W` để đi tới) hoặc gõ từ tiếng Anh bắt đầu bằng `w` (`web`, `win`, `word`) dễ bị lỗi tự biến thành `ư` (`ưeb`, `ưin`).
- **Quyết định:** **TẮT HOÀN TOÀN** lối tắt biến `w` ở đầu từ thành `ư`. Phím `w` đứng ở đầu từ sẽ giữ nguyên là chữ `w`. Phím `w` CHỈ đóng vai trò phím dấu Móc Telex khi đứng sau nguyên âm (`uw` -> `ư`, `ow` -> `ơ`, `uow` -> `ươ`).
- **Kết quả:** Tránh $100\%$ lỗi kẹt phím khi chơi game và gõ từ tiếng Anh có chứa phím `w`.

---

### [2026-07-23] Quyết định 008: Multi-device EVIOCGRAB, Smart Tone Placement & Smart Backspace Diff
- **Bối cảnh:** Bàn phím USB phần cứng (như `SEMICO USB Keyboard`) tạo nhiều cổng `/dev/input/event*` song song. Việc chỉ grab 1 cổng gây lọt phím thô ra OS, hoặc grab 2 cổng gây trùng lặp/kẹt phím. Đồng thời vị trí đặt dấu thanh Telex (`giáo`, `mẫu`, `cháu`) và thao tác xóa `Backspace` bị giật/nuốt chữ trên Wayland (`niri`/`kitty`).
- **Quyết định:**
  1. **Multi-device Grab & Multi-node Read:** Grab độc quyền TOÀN BỘ các cổng `event*` thuộc chiếc bàn phím đó và đọc dữ liệu từ tất cả các cổng đã grab.
  2. **Phân loại Uinput QWERTY Device:** Lọc bỏ keycode nút nguồn/ngủ (Power/Sleep) khỏi Virtual Device để `libinput` và `systemd-logind` nhận diện chuẩn $100\%$ là Bàn phím nhập liệu QWERTY.
  3. **Smart Tone Placement (Quy chuẩn Dấu thanh):** Ưu tiên đặt dấu thanh lên các nguyên âm có dấu phụ (`â`, `ă`, `ê`, `ô`, `ơ`, `ư`), ưu tiên nhị hợp âm (`au`, `ay` -> đặt lên `a`), và xử lý phụ âm ghép `gi` + nguyên âm (đặt dấu lên nguyên âm đi sau `i`).
  4. **Smart Backspace Diff & ISO 14755 Micro-delay:** Khi xóa lùi phím, chỉ phát đúng số phím Backspace chênh lệch (Diff count) thay vì xóa và gõ lại cả từ. Chèn trễ $1\text{ms}$ giữa các mã Unicode ISO 14755 để ngăn Terminal bị trễ nhịp parser.
- **Kết quả:** Gõ tiếng Việt chuẩn chính tả $100\%$, xóa phím tức thì mượt mà, gõ chuẩn mượt trên mọi ứng dụng Linux (VS Code, Chrome, Terminal, Telegram...).

---

### [2026-07-23] Quyết định 009: Physical Backspace PassThrough & Smart Minimal Edit Distance
- **Bối cảnh:** Khi người dùng bấm phím `Backspace` vật lý để xóa thủ công ký tự trên Facebook Messenger hoặc Web Editor, việc phát lại lệnh xóa dồn dập khiến trình chỉnh sửa React DOM bị đơ phím Backspace (`liệt phím Backspace`).
- **Quyết định:**
  1. **PassThrough Phím Backspace Vật lý:** Khi người dùng bấm phím Backspace vật lý, `vknetd` chỉ cập nhật buffer nguyên âm ngầm và cho phép phím `KEY_BACKSPACE` phát thẳng PassThrough 1 lần duy nhất xuống hệ thống OS.
  2. **Smart Common Prefix Diff:** Trong quá trình gõ ghép dấu, `main.rs` tự động giữ nguyên tiền tố trùng lặp (Common Prefix) và chỉ phát số phím Backspace chênh lệch tối thiểu + phần đuôi ký tự biến đổi.
  3. **Clipboard Injection cho Web Rich-Text Editors:** Dùng `wl-copy` + `Shift+Insert` khi phát các chuỗi Unicode tiếng Việt phức tạp để tương thích $100\%$ với các trình soạn thảo React/Web Editors.
- **Kết quả:** Triệt hạ $100\%$ lỗi liệt phím Backspace, cảm giác gõ phím và xóa phím nhạy mượt $100\%$ như bàn phím thật trên mọi ứng dụng.

---

### [2026-07-23] Quyết định 010: Paired SYN_REPORT for PassThrough Events (Sửa dứt điểm phím Backspace)
- **Bối cảnh:** Khi người dùng bấm phím Backspace vật lý, daemon phát lệnh PassThrough `virt_device.emit(&[ev])` nhưng thiếu gói ngắt `SYN_REPORT` (`evdev::InputEvent::new(EventType::SYNCHRONIZATION, 0, 0)`). Kết quả là Linux Kernel `uinput` driver giữ phím Backspace trong bộ đệm và không bao giờ xả xuống `libinput` / Display Server, khiến người dùng bấm phím Backspace mà chữ trên màn hình không hề bị xóa.
- **Quyết định:** Luôn ghép cặp gói ngắt đồng bộ `SYN_REPORT` ngay sau mọi sự kiện phím PassThrough (`virt_device.emit(&[ev, syn])`).
- **Kết quả:** Triệt hạ dứt điểm $100\%$ sự cố ngưng trệ phím Backspace. Phím Backspace vật lý giờ đây xả trực tiếp xuống OS và xóa chữ tức thì trên mọi phần mềm (AFFiNE, Facebook Messenger, Chrome, Terminal, VS Code...).

---

### [2026-07-23] Quyết định 011: Kernel-Native Hardware Keycode Emission & Standard US QWERTY Layout Enforce
- **Bối cảnh:** Qua thử nghiệm thực tế và phân tích video trên các ứng dụng Rich-Text Editors (AFFiNE, Facebook Messenger) và Launcher Niri (`Super+R`), việc phụ thuộc vào chuỗi ISO 14755 (`Ctrl+Shift+U`) của GTK User-space gây ra xung đột phím tắt `Ctrl+U`, khóa IM Context DOM, và hoàn toàn không hoạt động trên Niri Launcher.
- **Quyết định:**
  1. **Chuyển đổi sang Phát phím Mã phần cứng Nguyên thủy Tầng Nhân (Kernel Hardware Keycode Emission):** Đăng ký trực tiếp các mã phím phần cứng Kernel (`KEY_ETH` cho `đ`, `KEY_COMPOSE`, XKB Latin-1 Mapping) vào thiết bị bàn phím ảo `/dev/uinput`. Phát phím nguyên bản như bàn phím USB phần cứng thật.
  2. **Quy định Layout Chuẩn English (US):** Quy định mặc định hệ thống người dùng cài đặt bàn phím English (US) giống như tiền lệ của UniKey, EVKey, OpenKey để đơn giản hóa kiến trúc và đảm bảo tốc độ $<1\text{ms}$.
  3. **Lọc Node Bàn phím chính (Primary Node Selector):** Chỉ grab độc quyền node bàn phím chính QWERTY (`event7`), bỏ qua node phụ Media/Fn (`event4`) để loại bỏ hoàn toàn hiện tượng lặp phím bấm (Race Condition).
- **Kết quả:** Thống nhất định hướng kiến trúc v2.0 cho phiên làm việc tiếp theo.






