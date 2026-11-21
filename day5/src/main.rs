// Import trait `Display` từ thư viện chuẩn để có thể in giá trị ra màn hình bằng `{}`
// (khác với `{:?}` dùng cho trait `Debug`).
use std::fmt::Display;

// 1. ERROR HANDLING với Result<T, E> và toán tử `?`
// - `Result<T, E>` là kiểu enum chuẩn có sẵn:
//   + `Ok(T)`: xử lý thành công, trả về giá trị kiểu `T` (ở đây là `u8`).
//   + `Err(E)`: xử lý thất bại, trả về thông tin lỗi kiểu `E` (ở đây là `String`).
// - Hàm này nhận mã giảm giá dạng "SALE-20" và trả về phần trăm giảm giá hợp lệ (0..=100).
fn parse_discount(raw: &str) -> Result<u8, String> {
    // Tách chuỗi theo dấu '-' và gom kết quả thành `Vec<&str>`.
    let parts: Vec<&str> = raw.split('-').collect();

    // Nếu không tách ra đúng 2 phần (ví dụ chuỗi "INVALID"), báo lỗi ngay.
    if parts.len() != 2 {
        return Err(String::from("Sai định dạng mã (cần dạng PREFIX-NUMBER)"));
    }

    // Lấy phần thứ 2, xóa khoảng trắng thừa bằng `.trim()`, rồi `.parse()` sang `u8`.
    // - `.map_err(|_| ...)`: chuyển kiểu lỗi `ParseIntError` sang `String` cho khớp kiểu trả về của hàm;
    //   `|_|` nghĩa là không dùng tới chi tiết lỗi gốc.
    // - Toán tử `?` ở cuối: nếu là `Err`, lập tức trả `Err` ra khỏi hàm; nếu là `Ok(n)`, tự lấy `n` gán cho `value`.
    let value: u8 = parts[1]
        .trim()
        .parse()
        .map_err(|_| String::from("Phần giá trị không phải số hợp lệ (0..=255)"))?;

    // Kiểm tra quy tắc nghiệp vụ: phần trăm giảm giá không được vượt quá 100.
    if value > 100 {
        return Err(String::from("Mức giảm giá không được vượt quá 100%"));
    }

    // Trả về kết quả hợp lệ bọc trong `Ok`.
    Ok(value)
}

// Hàm gọi `parse_discount` và dùng `?` để lan truyền lỗi lên trên nếu có.
fn calculate_final_price(price: u32, raw_code: &str) -> Result<u32, String> {
    // Nếu `parse_discount` trả về `Err`, hàm này cũng dừng sớm và trả `Err` đó ra ngoài.
    let discount = parse_discount(raw_code)?;

    // Ép `discount` từ `u8` sang `u32` để tính toán cùng kiểu với `price: u32`.
    let discounted_amount = price * (discount as u32) / 100;

    // `saturating_sub` trừ an toàn: nếu số tiền giảm lớn hơn `price` thì dừng ở 0, không bị lỗi tràn số âm.
    Ok(price.saturating_sub(discounted_amount))
}

// 2. GENERICS (`<T>`) & TRAIT BOUNDS (`PartialOrd + Display`)
// - `<T>` là tham số kiểu generic: cho phép hàm dùng chung cho nhiều kiểu dữ liệu (`u32`, `f64`, `&str`, ...).
// - `PartialOrd + Display` là trait bound (ràng buộc năng lực của kiểu `T`):
//   + `PartialOrd`: kiểu `T` phải so sánh lớn/nhỏ được bằng toán tử `>`.
//   + `Display`: kiểu `T` phải in được bằng `{}`.
// - `items: &[T]` nhận slice (mượn để đọc, không chiếm quyền sở hữu mảng/vector gốc).
fn print_largest<T: PartialOrd + Display>(label: &str, items: &[T]) {
    // Kiểm tra rỗng trước khi truy cập `items[0]` để tránh lỗi truy cập ngoài phạm vi (out of bounds).
    if items.is_empty() {
        println!("[{}] Danh sách rỗng", label);
        return;
    }

    // `largest` giữ tham chiếu (`&T`) tới phần tử lớn nhất tạm thời (bắt đầu từ phần tử đầu tiên).
    let mut largest = &items[0];

    // Duyệt từ phần tử thứ 2 đến hết (`&items[1..]`).
    for item in &items[1..] {
        // So sánh giá trị mà `item` và `largest` trỏ tới bằng `>`.
        if item > largest {
            largest = item;
        }
    }

    // In ra giá trị lớn nhất tìm được.
    println!("[{}] Giá trị lớn nhất: {}", label, largest);
}

fn main() {
    // Danh sách các bộ test: (giá gốc, mã giảm giá).
    let cases = [
        (500_000, "SALE-20"),
        (500_000, "INVALID"),
        (500_000, "SALE-150"),
    ];

    // Duyệt qua từng case và dùng `match` để xử lý đủ cả 2 nhánh `Ok` và `Err`.
    for (price, code) in cases {
        match calculate_final_price(price, code) {
            Ok(final_price) => println!("Giá sau giảm ({code}): {final_price} VND"),
            Err(err_msg) => println!("Lỗi khi xử lý ({code}): {err_msg}"),
        }
    }

    // `unwrap_or_else`: nếu gặp `Err`, chạy closure bên trong để log lỗi và trả về giá trị dự phòng `300_000`.
    let fallback_price = calculate_final_price(300_000, "BAD-CODE").unwrap_or_else(|err| {
        println!("Dùng giá gốc vì lỗi: {err}");
        300_000
    });
    println!("Giá chốt: {fallback_price} VND");

    // Gọi cùng một hàm generic `print_largest` cho 3 kiểu dữ liệu khác nhau.
    let order_totals = vec![120_000_u32, 450_000, 290_000, 990_000];
    let ratings = vec![4.5_f64, 4.9, 3.8, 4.7];
    let tags = vec!["alpha", "omega", "beta"];

    print_largest("Đơn hàng (VND)", &order_totals);
    print_largest("Đánh giá sao", &ratings);
    print_largest("Nhãn", &tags);
}