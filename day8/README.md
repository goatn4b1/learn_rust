# Ngày 8: Làm việc song song nhiều luồng an toàn (`Arc` & `Mutex`)

Bài hôm nay giải quyết một bài toán rất thực tế: khi có **nhiều nhân viên cùng xử lý công việc song song** (đa luồng), làm sao để họ cùng cập nhật chung một con số mà không bị tính sai hoặc đụng độ nhau?

---

## 1. Thread — Nhiều "nhân viên" chạy việc song song
- Thay vì chỉ làm tuần tự từng việc một trên một luồng duy nhất, `thread::spawn` mở thêm các luồng chạy song song để chia sẻ khối lượng công việc.

---

## 2. `Arc<T>` — Chia sẻ quyền truy cập an toàn giữa các luồng
- Nếu `Rc` dùng cho một luồng, thì `Arc` dùng khi **nhiều luồng cùng giữ tham chiếu tới chung một món đồ** và đảm bảo bộ đếm số người giữ được cập nhật an toàn.

---

## 3. `Mutex<T>` — Chiếc khóa "mỗi lần chỉ một người được sửa"
- Khi nhiều luồng cùng muốn cộng thêm vào một biến đếm, nếu không có khóa thì hai luồng có thể đọc cùng một số cũ và ghi đè làm mất kết quả.
- `Mutex` giống như **chiếc chìa khóa duy nhất** gắn với món đồ: ai muốn sửa phải cầm khóa (`.lock()`), sửa xong thì trả khóa để người tiếp theo vào.

---

👉 Xem code chạy thực tế tại: [`src/main.rs`](./src/main.rs)