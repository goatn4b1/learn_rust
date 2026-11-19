# Ngày 3: Structs, Enums, `impl`, Slices & Pattern Matching

Bài này làm một ví dụ nhỏ về **quản lý đơn hàng** (chờ thanh toán, đã trả tiền, bị hủy) và **cắt lấy một đoạn chữ** để làm quen cách tổ chức dữ liệu trong Rust.

## 1. `enum` — Các trạng thái có thể xảy ra của đơn hàng
- Giống như tờ phiếu chọn trạng thái: **Đang chờ (`Pending`)**, **Đã thanh toán (`Paid` kèm số tiền)**, hoặc **Đã hủy (`Cancelled` kèm lý do)**.
- Mỗi trạng thái mang đúng thông tin riêng của nó nên tránh được dữ liệu thừa hoặc mâu thuẫn trạng thái.

## 2. `struct` — Tờ phiếu thông tin gộp chung của 1 đơn hàng
- Gom các thông tin liên quan (`id`, `code`, `status`) vào chung một `Order`.
- Khi biến được đánh dấu `mut`, các trường bên trong mới có thể cập nhật được.

## 3. `impl` — Các hành động gắn với đơn hàng
- `new`: Hàm khởi tạo đơn mới (mặc định đang chờ thanh toán).
- `summary(&self)`: Mượn để đọc tóm tắt thông tin, không sửa dữ liệu.
- `mark_paid(&mut self, ...)` & `cancel(&mut self, ...)`: Mượn khả biến để cập nhật trạng thái đơn.

## 4. Slice (`&str`) — Cắt lấy một đoạn chữ để xem nhanh
- Lấy một lát cắt của chuỗi mà không cần tạo bản sao mới tốn bộ nhớ.
- Lưu ý: cắt theo byte nên cần cẩn thận với chuỗi tiếng Việt có dấu hoặc emoji.

## 5. `match` & `Option` — Chia nhánh xử lý & tránh tình trạng "trống trơn"
- `Option`: Giúp xử lý rõ ràng hai trường hợp có dữ liệu (`Some`) hoặc không có (`None`) thay cho `null`.
- `match`: Ép xử lý đủ các nhánh để khó bị sót trường hợp.

👉 Xem code đầy đủ tại: [`src/main.rs`](./src/main.rs)