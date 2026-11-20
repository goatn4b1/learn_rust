# Ngày 4: Chia nhỏ code & Lưu trữ nhiều dữ liệu

Bài hôm nay làm quen với hai việc rất hay gặp khi viết chương trình lớn hơn:
1. **Chia nhỏ code thành từng khu vực (`mod` & `pub`)** để không phải nhét tất cả vào một chỗ rối rắm.
2. **Lưu trữ nhiều món dữ liệu cùng lúc** (danh sách số, chuỗi chữ, và bảng tra cứu theo mã).

---

## 1. `mod` và `pub` — Chia ngăn kéo & Quy định ai được mở ngăn nào
- **`mod` (Module):** Giống như chia một chiếc tủ lớn thành nhiều ngăn nhỏ — ví dụ ngăn `math` chuyên tính toán, ngăn `text` chuyên xử lý chữ.
- **`pub` (Public):** Mặc định các món đồ bên trong ngăn đều được **khóa lại** (chỉ dùng nội bộ trong ngăn đó). Khi thêm `pub`, bạn mới mở khóa cho bên ngoài gọi tới được.

---

## 2. `Vec` (Vector) — Một danh sách có thể dài thêm
- Giống như một tờ giấy ghi danh sách điểm số: lúc đầu có 3 số `[10, 20, 30]`, sau đó bạn có thể **thêm tiếp** số `50`, `100` vào cuối danh sách.
- Khi lấy một ô bất kỳ bằng `.get(2)`, kết quả có thể **có giá trị** (`Some`) hoặc **không có** (`None` nếu vượt quá độ dài danh sách) — giúp tránh lỗi bất ngờ.

---

## 3. `String` — Chuỗi chữ có thể nối thêm
- Khác với đoạn chữ cố định chỉ xem tạm, `String` là kiểu chữ **thuộc quyền sở hữu của biến** và có thể nối thêm chữ mới (như `.push_str(...)` hay `.push(...)`).
- Lưu ý nhỏ: khi dùng dấu `+` để nối hai chuỗi theo kiểu `s1 + &s2`, `s1` sẽ **chuyển quyền sở hữu** sang chuỗi mới nên không gọi lại `s1` nữa.

---

## 4. `HashMap` — Sổ tra cứu theo cặp "Mã <-> Thông tin"
- Giống như danh bạ hoặc bảng tra cứu: bạn lưu theo từng cặp, ví dụ `ID 1 -> Alice`, `ID 2 -> Bob`, `ID 3 -> Charlie`.
- Khi cần tìm theo `ID`, bạn tra nhanh qua `.get(...)` — nếu có sẽ trả về tên tương ứng, không có thì báo không tìm thấy.

---

👉 Xem code chạy thực tế tại: [`src/main.rs`](./src/main.rs)