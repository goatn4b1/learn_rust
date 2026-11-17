const MAX_CONNECTIONS: u32 = 10_000;
// Hằng số: Bắt buộc phải khai báo kiểu dữ liệu
// Global Scope (Lưu ý: 'let' và 'let mut' không được phép khai báo ở Global để tránh lỗi xung đột đa luồng)
// Tối ưu tốc độ (Copy thẳng giá trị vào nơi gọi lúc compile, không tốn ô nhớ Stack)

fn main() {
    // Immutable: Không thể sửa đổi, an toàn tuyệt đối 
    let a = 10; 
    // a = 20; -> Đổi giá trị = Lỗi 

    // Mutable: Thêm 'mut' nhưng không đổi được kiểu dữ liệu
    let mut b = 10;
    b = 20; // Không bị lỗi
    // b = "hello"; -> Đổi kiểu dữ liệu = Lỗi (vì b đã cố định kiểu i32 số nguyên)

    // Shadowing: Tạo biến trùng tên, CÓ THỂ đổi kiểu dữ liệu
    let name = "Goat";       // Đang là kiểu chuỗi (&str)
    let name = name.len();   // Đổi sang kiểu số nguyên (usize) lấy độ dài chuỗi (= 4)

    println!("a: {}, b: {}, name length: {}, max: {}", a, b, name, MAX_CONNECTIONS);

    // Hệ thống kiểu dữ liệu sát phần cứng:
    // - Số nguyên (Signed - cả số âm và dương): i8, i16, i32 (mặc định), i64, i128, isize (theo kiến trúc CPU 32/64-bit)
    // - Số nguyên không dấu (>= 0): u8, u16, u32, u64, u128, usize
    // - Số thực: f32, f64 (mặc định)
    // - Boolean & Ký tự: bool (1 byte), char (4 bytes - hỗ trợ đầy đủ Unicode / Emoji)
    
    // Bộ nhớ Stack và Heap:
    let x = 42; // Stack (i32 cố định 4 bytes)
    let mut y = String::from("Hello"); // Khởi tạo kiểu String: tách làm 2 phần (Stack giữ con trỏ, Heap giữ chữ)
    y.push_str(" Goat!"); // Mở rộng vùng nhớ trên Heap

    println!("x: {}, y: {}", x, y);

    // BẢN CHẤT BỘ NHỚ CỦA x VÀ y:
    // 1. Biến x: Kiểu i32 có kích thước cố định (4 bytes) lúc biên dịch -> Lưu thẳng vào Stack.
    // 2. Biến y: Kiểu String có kích thước động (có thể dài thêm 10 hoặc 10,000 chữ lúc chạy) 
    //    nên ngay khi khởi tạo đã được tách làm 2 phần nằm ở 2 nơi khác nhau:
    //    - Trên Stack: Lưu thông tin quản lý (con trỏ địa chỉ trỏ sang Heap, độ dài chuỗi, sức chứa).
    //    - Trên Heap: Lưu giá trị chuỗi thực tế ("Hello Goat!") nằm liền mạch nhau. 
    //      Khi gọi push_str, nếu ô nhớ sát cạnh "Hello" không đủ chỗ trống, hệ điều hành sẽ 
    //      chuyển cả chuỗi sang địa chỉ mới rộng hơn, xóa chỗ cũ và cập nhật lại địa chỉ ở Stack.
} // <-- Sau khi chạy hết hàm (ra khỏi phạm vi scope), Rust tự động gọi drop(y) giải phóng vùng nhớ trên Heap ngay lập tức!