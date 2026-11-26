use std::sync::{Arc, Mutex};
use std::thread;

// 1. CONCURRENCY: `Arc<T>` + `Mutex<T>` + `thread::spawn`
// - `Arc<T>` (Atomic Reference Counted): giống `Rc<T>` nhưng an toàn khi chia sẻ qua nhiều thread.
// - `Mutex<T>` (Mutual Exclusion): khóa bảo vệ dữ liệu bên trong, chỉ cho 1 thread mở khóa
//   và sửa dữ liệu tại một thời điểm.
// - `move` closure: chuyển quyền sở hữu biến (thường là bản clone của `Arc`) vào bên trong thread.

fn main() {
    // Bộ đếm số đơn đã xử lý, bọc trong `Mutex` để khóa an toàn và bọc trong `Arc` để chia sẻ đa luồng.
    let processed_orders = Arc::new(Mutex::new(0_u32));

    // Lưu các handle của thread để chờ tất cả hoàn thành trước khi kết thúc `main`.
    let mut handles = Vec::new();

    for worker_id in 1..=4 {
        // Clone `Arc` cho từng worker thread (chỉ tăng bộ đếm nguyên tử, không nhân bản dữ liệu gốc).
        let counter_ref = Arc::clone(&processed_orders);

        let handle = thread::spawn(move || {
            // Mỗi worker giả lập xử lý 5 đơn hàng.
            for _ in 0..5 {
                // `.lock()` xin khóa truy cập; dùng `.unwrap()` đơn giản cho ví dụ cơ bản.
                // Khóa (`MutexGuard`) sẽ tự động mở khi biến `guard` ra khỏi scope.
                let mut count = counter_ref.lock().unwrap();
                *count += 1;
            }
            println!("Worker #{worker_id} đã hoàn thành phần việc.");
        });

        handles.push(handle);
    }

    // Chờ tất cả các thread chạy xong (`join`).
    for handle in handles {
        handle.join().unwrap();
    }

    // Đọc giá trị cuối cùng sau khi mọi thread đã xong.
    let final_count = *processed_orders.lock().unwrap();
    println!("Tổng số đơn đã xử lý an toàn: {final_count}");
}