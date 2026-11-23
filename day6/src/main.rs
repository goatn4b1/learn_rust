// 1. LIFETIMES (`'a`)
// - Lifetime không làm dữ liệu sống lâu hơn, mà mô tả mối quan hệ phạm vi sống giữa các tham chiếu (`&`).
// - Ở đây, tham chiếu trả về sẽ sống ít nhất bằng phần giao nhau của `a` và `b`.
fn longer_text<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

#[derive(Debug)]
struct OrderItem {
    name: String,
    price: u32,
    quantity: u32,
    in_stock: bool,
}

impl OrderItem {
    fn subtotal(&self) -> u32 {
        self.price * self.quantity
    }
}

fn main() {
    // --- Phần A: Lifetime `'a` ---
    let code_a = String::from("ORD-100");
    let best_code: &str;
    {
        let code_b = String::from("ORD-2026-VIP");
        // `best_code` mượn dữ liệu từ `code_a` hoặc `code_b`, nên phải dùng trước khi `code_b` ra khỏi block `{}`.
        best_code = longer_text(&code_a, &code_b);
        println!("Mã dài hơn (trong scope): {best_code}");
    }
    // Nếu gọi `best_code` ở đây sau khi `code_b` đã bị drop, compiler sẽ chặn ngay để tránh dangling reference.

    // --- Phần B: Iterators & Closures ---
    // Closure `|...| ...` là hàm nặc danh gọn nhẹ, có thể bắt biến từ môi trường xung quanh.
    let tax_multiplier: u32 = 10; // Ví dụ phụ phí/thuế cố định theo đơn vị đơn giản cho bài demo

    let items = vec![
        OrderItem {
            name: String::from("Bàn phím cơ"),
            price: 1_200_000,
            quantity: 1,
            in_stock: true,
        },
        OrderItem {
            name: String::from("Chuột không dây"),
            price: 650_000,
            quantity: 2,
            in_stock: true,
        },
        OrderItem {
            name: String::from("Tai nghe"),
            price: 900_000,
            quantity: 1,
            in_stock: false,
        },
    ];

    // `.iter()` duyệt qua tham chiếu từng phần tử (không làm mất quyền sở hữu `items`).
    // `.filter(|item| item.in_stock)` giữ lại các món còn hàng.
    // `.map(|item| item.subtotal() + tax_multiplier)` biến đổi từng món thành thành tiền.
    // `.collect()` gom kết quả thành một `Vec<u32>`.
    let active_totals: Vec<u32> = items
        .iter()
        .filter(|item| item.in_stock)
        .map(|item| item.subtotal() + tax_multiplier)
        .collect();

    println!("Danh sách thành tiền các món còn hàng: {:?}", active_totals);

    // `.sum()` tính tổng nhanh trên iterator
    let grand_total: u32 = active_totals.iter().sum();
    println!("Tổng cộng: {grand_total} VND");
}