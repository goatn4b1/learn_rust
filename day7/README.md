# Ngày 7: Smart Pointers (`Box`, `Rc`) & Viết kiểm thử tự động (`Unit Test`)

Bài hôm nay làm quen với hai thứ giúp chương trình chạy ổn định và dễ kiểm soát khi lớn lên:
1. **Cách cất giữ và chia sẻ dữ liệu thông minh hơn** (`Box` và `Rc`).
2. **Cách nhờ máy tính tự chạy kiểm tra xem logic có đúng không** (`#[test]` & `cargo test`).

---

## 1. `Box<T>` — Hộp đựng dữ liệu trên vùng nhớ Heap
- Bình thường biến nhỏ nằm gọn ở ngăn xếp làm việc nhanh (`stack`), còn khi bạn muốn **đặt rõ dữ liệu vào hộp trên `Heap`** hoặc cần một kích thước cố định rõ ràng, bạn dùng `Box::new(...)`.
- `Box` chỉ có **một chủ sở hữu** tại một thời điểm — hết phạm vi dùng là tự dọn dẹp gọn gàng.

---

## 2. `Rc<T>` — Chia sẻ dữ liệu cho nhiều nơi cùng đọc & tự đếm số người đang dùng
- Khác với `Box` (chỉ một chủ), `Rc` (Reference Counted) cho phép **nhiều chỗ cùng cầm tham chiếu** tới chung một dữ liệu (ví dụ nhiều đơn hàng cùng đọc chung một bảng quy định phí ship).
- Mỗi lần gọi `Rc::clone(...)`, bộ đếm người dùng tăng lên 1; khi một chỗ dùng xong và thoát ra, bộ đếm giảm đi 1. Đến khi không còn ai dùng (`0`), dữ liệu mới được giải phóng.

---

## 3. Unit Test — Tự kiểm tra logic thay vì phải dò tay từng lần
- Thay vì mỗi lần sửa công thức tính tiền lại phải sửa `main` và nhìn mắt thường xem đúng chưa, bạn viết các hàm kiểm tra gắn `#[test]`.
- Khi gõ `cargo test` trong terminal, Rust sẽ tự chạy các bài test và báo ngay cái nào **đỗ (`ok`)**, cái nào **trượt (`FAILED`)**.

---

👉 Xem code và bài test thực tế tại: [`src/main.rs`](./src/main.rs)