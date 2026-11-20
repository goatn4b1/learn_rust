// 1. KHAI BÁO MODULE (Ví dụ mô phỏng cấu trúc file)
// Giả sử bạn có một file math_utils.rs và string_helper.rs nằm trong thư mục con tên là utils
// Nhưng để code chạy trong main.rs đơn giản, ta định nghĩa inline mod:

mod utils {
    // --- Module con bên trong ---
    pub mod math {
        // Hàm này có thể gọi từ bên ngoài nhờ từ khóa 'pub'
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        // Hàm này là private, bên ngoài không gọi được
        #[allow(dead_code)]
        fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }
    }

    pub mod text {
        pub fn capitalize(s: &str) -> String {
            if s.is_empty() {
                String::new()
            } else {
                let (first, rest) = s.split_at(1);
                format!("{}{}", first.to_uppercase(), rest)
            }
        }
    }
}

// --- Main function ---

fn main() {
    // 2. SỬ DỤNG MODULE
    // Gọi hàm từ module con: crate::utils::math::add
    let result = utils::math::add(10, 20);
    println!("Kết quả cộng (mod): {}", result);

    let text_upper = utils::text::capitalize("goat");
    println!("Chữ hoa đầu (mod): {}", text_upper);


    println!("\n--- 3. CÁC COLLECTIONS PHỔ BIẾN ---");

    // Vector (Vec<T>)
    println!("--- Vector ---");
    let mut scores = vec![10, 20, 30];
    scores.push(50);
    scores.push(100);

    // In ra toàn bộ phần tử dùng vòng lặp
    print!("Vector scores: ");
    for score in &scores {
        print!("{} ", score);
    }
    println!();

    // Lấy phần tử an toàn
    match scores.get(2) {
        Some(val) => println!("Giá trị tại index 2 là: {}", val),
        None => println!("Không có giá trị tại index 2"),
    }


    // String (Owned String)
    println!("\n--- String ---");
    let s1 = String::from("Hello");
    let s2 = String::from(" Rust");
    let s3 = s1 + &s2; // s1 bị move ở đây, không dùng được s1 nữa
    // println!("{}", s1); // -> Lỗi Ownership

    let mut greeting = String::from("Welcome");
    greeting.push_str(" to the");
    greeting.push(' ');
    greeting.push_str("Rust world!");
    println!("{}", greeting);


    // HashMap
    println!("\n--- HashMap ---");
    use std::collections::HashMap; // Phải import HashMap vào scope

    let mut users = HashMap::new();

    // Insert (Key - Value)
    users.insert(1, String::from("Alice"));
    users.insert(2, String::from("Bob"));
    users.insert(3, String::from("Charlie"));

    // Get (Trả về Option<&String>)
    let user_id_2 = 2;
    if let Some(name) = users.get(&user_id_2) {
        println!("User ID {} là: {}", user_id_2, name);
    } else {
        println!("Không tìm thấy User ID {}", user_id_2);
    }

    // Lặp qua HashMap
    println!("Danh sách users:");
    for (id, name) in &users {
        println!("ID: {}, Name: {}", id, name);
    }
}