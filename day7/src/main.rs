use std::rc::Rc;

// 1. SMART POINTERS: Box<T> & Rc<T>
// - `Box<T>` cấp phát dữ liệu trên Heap và giữ quyền sở hữu duy nhất (chỉ có 1 owner tại một thời điểm).
// - `Rc<T>` (Reference Counted) cho phép nhiều nơi cùng chia sẻ quyền đọc một dữ liệu trên Heap
//   (dùng trong single-thread); bộ đếm `strong_count` tăng khi `.clone()` và giảm khi ra khỏi scope.
#[derive(Debug)]
struct StorePolicy {
    currency: String,
    free_shipping_threshold: u32,
}

impl StorePolicy {
    fn shipping_fee(&self, subtotal: u32) -> u32 {
        if subtotal >= self.free_shipping_threshold {
            0
        } else {
            25_000
        }
    }
}

// Hàm tính tổng đơn hàng dùng `Box<u32>` cho ví dụ đóng gói giá trị trên Heap
// và `Rc<StorePolicy>` để nhiều nơi cùng tham chiếu chính sách cửa hàng.
fn calculate_order_total(items_subtotal: Box<u32>, policy: &Rc<StorePolicy>) -> u32 {
    let base = *items_subtotal;
    let fee = policy.shipping_fee(base);
    base + fee
}

fn main() {
    // Khởi tạo chính sách cửa hàng bọc trong `Rc` để chia sẻ giữa nhiều bộ phận/đơn hàng
    let policy = Rc::new(StorePolicy {
        currency: String::from("VND"),
        free_shipping_threshold: 500_000,
    });
    println!(
        "Số lượng tham chiếu ban đầu ({}): {}",
        policy.currency,
        Rc::strong_count(&policy)
    );

    // Tạo hai đơn hàng cùng dùng chung `policy` qua `Rc::clone(&policy)`
    // (lưu ý: `Rc::clone` chỉ tăng biến đếm tham chiếu, không copy sâu toàn bộ struct).
    let order1_policy = Rc::clone(&policy);
    let order1_subtotal = Box::new(320_000_u32);
    let order1_total = calculate_order_total(order1_subtotal, &order1_policy);
    println!(
        "Đơn 1 tổng: {order1_total} {} | strong_count hiện tại: {}",
        order1_policy.currency,
        Rc::strong_count(&policy)
    );

    let order2_total = {
        let order2_policy = Rc::clone(&policy);
        let order2_subtotal = Box::new(650_000_u32);
        let total = calculate_order_total(order2_subtotal, &order2_policy);
        println!(
            "Trong scope đơn 2 -> strong_count: {}",
            Rc::strong_count(&policy)
        );
        total
    }; // `order2_policy` kết thúc ở đây -> `strong_count` giảm đi 1

    println!("Đơn 2 tổng: {order2_total} {}", policy.currency);
    println!(
        "Sau khi đơn 2 kết thúc -> strong_count: {}",
        Rc::strong_count(&policy)
    );
}

// 2. UNIT TESTS
// - Module `tests` chỉ được biên dịch và chạy khi bạn gõ `cargo test`.
// - `#[test]` đánh dấu một hàm là bài kiểm thử tự động.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn charges_shipping_below_threshold() {
        let policy = Rc::new(StorePolicy {
            currency: String::from("VND"),
            free_shipping_threshold: 500_000,
        });
        let total = calculate_order_total(Box::new(300_000), &policy);
        assert_eq!(total, 325_000);
    }

    #[test]
    fn free_shipping_at_or_above_threshold() {
        let policy = Rc::new(StorePolicy {
            currency: String::from("VND"),
            free_shipping_threshold: 500_000,
        });
        let total = calculate_order_total(Box::new(500_000), &policy);
        assert_eq!(total, 500_000);
    }
}