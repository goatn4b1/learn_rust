# Ngày 6: Thời hạn tham chiếu (`Lifetime`) & Chuỗi xử lý danh sách (`Iterator` + `Closure`)

Bài hôm nay giải quyết hai chuyện rất thực tế:
1. **Đảm bảo "giấy ghi chú địa chỉ" (`&`) không trỏ vào món đồ đã bị vứt đi** (`Lifetime`).
2. **Lọc và tính toán trên một danh sách** theo từng bước nối tiếp nhau (`Iterator` & `Closure`).

---

## 1. Lifetime (`'a`) — Kiểm tra món đồ còn tồn tại không
- Khi bạn chỉ **chỉ chỗ** tới một món đồ chứ không cầm hẳn nó, món đồ đó phải còn tồn tại khi bạn nhìn tới chỗ đó.
- Ký hiệu `'a` giống như một cái mốc thời gian để máy tính kiểm tra xem chỗ tham chiếu có còn hợp lệ hay không, giúp tránh lỗi trỏ nhầm vào vùng nhớ đã giải phóng.

---

## 2. Closure — Một "công thức nhỏ viết nhanh ngay tại chỗ"
- Giống như khi bạn cần một quy tắc lọc nhanh (ví dụ: *"chỉ giữ món còn hàng"*) mà không cần đặt tên hàm riêng ở xa.
- Dạng viết gọn là `|item| item.in_stock`: đưa từng món `item` vào, kiểm tra xem còn hàng không.

---

## 3. Iterator — Dây chuyền xử lý từng món trong danh sách
- Thay vì tự viết vòng lặp dài dòng rồi tự tạo danh sách mới, bạn xếp các bước thành một dây chuyền:
  1. `.iter()`: Lấy từng món ra để xem.
  2. `.filter(...)`: Lọc giữ lại món đạt điều kiện.
  3. `.map(...)`: Tính ra con số mong muốn cho từng món.
  4. `.collect()`: Gom tất cả kết quả lại thành một danh sách mới.

---

👉 Xem code chạy thực tế tại: [`src/main.rs`](./src/main.rs)