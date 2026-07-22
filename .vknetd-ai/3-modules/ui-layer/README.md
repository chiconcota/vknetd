# UI & IPC LAYER MODULE

## 📌 Chức năng
Quản lý giao diện người dùng và giao tiếp IPC:
1. Unix Domain Socket Server (`/run/user/$UID/vknetd.sock`) nhận lệnh điều khiển từ UI.
2. System Tray Icon nhẹ nhàng hiển thị trạng thái `[ VIE ]` / `[ ENG ]`.
3. Giao diện GUI cài đặt (nếu có) bằng Tauri / GTK.
