fn main() {
    // 1. OWNERSHIP & MOVE (Dữ liệu trên Heap)
    let s1 = String::from("Goat");
    let s2 = s1; // s1 chuyển (Move) quyền sở hữu sang s2 -> s1 không gọi được
    // println!("{}", s1);
    println!("s2: {}", s2); // s2: Goat

    //2. Clone ( sao chép toàn bộ dữ liệu trên ô nhớ Heap và giữ bản gốc)
    let s3 = s2.clone();
    println!("s2: {}, s3 (sau clone): {}", s2, s3); // s2: Goat, s3 (sau clone): Goat

    //3. Copy ( Dữ liệu trên ô nhớ Stack có kích thước cố định)
    let x = 42;
    let y = x; // Các kiểu dữ liệu cơ bản tự động Copy, x vẫn dùng bình thường

    let mut z = 36; // Với các biến có thể thay đổi giá trị
    let z1 = z; // Sao chép giá trị 36 sang z1 (z1 mặc định là immutable) không thể sửa 

    z = 100; // Hoàn toàn hợp lệ! z đổi thành 100
    // z1 = 200;  // Lỗi! Vì z1 mặc định không có mut, không được gán lại giá trị mới
    println!("x: {}, y: {}, z: {}, z1: {}", x, y, z, z1); // x: 42, y: 42, z: 100, z1: 36

    // 4. BORROWING & REFERENCES (Mượn dữ liệu qua tham chiếu &)
    let mut message = String::from("Hello");

    // Mượn bất biến (&String) để đọc
    // Chỉ cho phép đọc dữ liệu và k thể sửa gái trị của biến gốc 
    // Có thể tạo ra nhiều lượt mượn bất biến tùy ý 
    print_message(&message);
    // = let r1 = &message;
    // let r2 = &message; // Mượn lần 2 (Đọc được luôn, không lỗi)
    //println!("Đang đọc nội dung mượn: {}", s);

    // Mượn khả biến (&mut String) để sửa đổi
    // Cho phép vừa đọc vừa sửa đổi gái trị của biến gốc
    // Tại 1 thời điểm chỉ duy nhất 1 mượn khả biến được phép tồn tại với 1 biến
    // K thể mượn song song
    // Lí do: Rust làm vậy để triệt tiêu lỗi Data Race ( Nhiều luồng cùng truy cập vào 1 ô nhớ trong đó ít nhất 1 luồng đang ghi dẫn đến lệch dữ liệu) ngay từ lúc biên dịch
    change_message(&mut message);
    // = let r2 = &mut message;
    // let r3 = &message; // Lỗi biên dịch! Đang có r2 mượn sửa không được phép mượn đọc cùng lúc
    // r2.push_str(" Goat!");
    println!("Sau khi mượn để sửa: {}", message);
}

// Hàm nhận tham chiếu mượn để đọc (Immutable borrow)
fn print_message(s: &String) {
    println!("Đang đọc nội dung mượn: {}", s);
}

// Hàm nhận tham chiếu mượn để sửa (Mutable borrow)
fn change_message(s: &mut String) {
    s.push_str(" Goat!");
}