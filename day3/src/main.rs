// 1. ENUM: Biểu diễn trạng thái & dữ liệu đi kèm
// - Thiết kế các trạng thái cho đơn hàng: `Pending`, `Paid { amount: u32 }`, `Cancelled(String)`.
// - Mỗi variant có thể mang kiểu dữ liệu riêng (không có field dư thừa khi sai trạng thái), giúp code chặt chẽ và khi dùng `match` sẽ bắt xử lý đủ nhánh, khó sót case.

#[derive(Debug)]
// - `#[derive(Debug)]` là attribute / procedural macro tự động sinh code cài đặt trait `Debug` cho `struct` hoặc `enum`.
// - Dùng để in dữ liệu khi debug qua `{:?}` hoặc `{:#?}` (in đẹp, xuống dòng thụt lề) mà không phải viết tay boilerplate `impl fmt::Debug`.
enum OrderStatus {
    Pending,
    Paid { amount: u32 },
    Cancelled(String),
}

// 2. STRUCT: Đóng gói dữ liệu có cấu trúc
// - Gom các trường dữ liệu liên quan thành một kiểu thống nhất (không có kế thừa như OOP truyền thống).
// - Mutability đi theo cả biến (`let mut order = ...` thì sửa được các field); ownership của từng field tuân theo quy tắc chung (`code: String` sẽ move/drop theo `order`).
// - Tách biệt dữ liệu (`struct`) với hành vi (`impl`).
#[derive(Debug)]
struct Order {
    id: u64,
    code: String,
    status: OrderStatus,
}

// 3. IMPL: Gắn associated function (hàm khởi tạo) và methods
// - Associated function (`fn new(...) -> Self`): không có `self`, gọi qua tên kiểu `Order::new(...)`; `Self` (viết hoa) là bí danh cho kiểu `Order`.
// - Method `&self`: mượn bất biến để đọc dữ liệu, gọi xong vẫn dùng tiếp biến gốc bình thường.
// - Method `&mut self`: mượn khả biến để sửa trạng thái bên trong; biến gọi bắt buộc khai báo `let mut` và tuân thủ quy tắc chỉ có 1 mượn khả biến tại một thời điểm.
// - Rust tự động tham chiếu/hủy tham chiếu (auto-ref/auto-deref) khi gọi `order.summary()` hay `order.mark_paid(...)`.
impl Order {
    // Hàm khởi tạo (không nhận self)
    fn new(id: u64, code: &str) -> Self {
        Self {
            id,
            code: String::from(code),
            status: OrderStatus::Pending,
        }
    }

    // Mượn bất biến (&self): Chỉ đọc dữ liệu
    fn summary(&self) {
        println!("Order #{} [{}]: {:?}", self.id, self.code, self.status);
    }

    // Mượn khả biến (&mut self): Cập nhật trạng thái
    fn mark_paid(&mut self, amount: u32) {
        self.status = OrderStatus::Paid { amount };
    }

    fn cancel(&mut self, reason: &str) {
        self.status = OrderStatus::Cancelled(String::from(reason));
    }
}

// 4. SLICE (&str): Tham chiếu tới một đoạn dữ liệu liền kề mà không chiếm quyền sở hữu
// - `&str` (string slice) trỏ tới một lát cắt của chuỗi (`&text[..len]`) hoặc chuỗi hằng/tĩnh mà không cần cấp phát mới trên Heap.
// - Hàm nhận `&str` nhận được cả `&String` lẫn string literal (`"RUST-DAY-3"`), linh hoạt hơn ép nhận `&String`.
// - Lưu ý: `text.len()` tính theo byte, cắt giữa ký tự UTF-8 nhiều byte (tiếng Việt/emoji) sẽ gây panic lúc chạy.
fn print_prefix(text: &str, len: usize) {
    let slice = if len <= text.len() { &text[..len] } else { text };
    println!("Prefix slice: {}", slice);
}

// 5. MATCH & OPTION: Khớp mẫu toàn diện và xử lý an toàn thay cho null
// - Rust không có `null`; dùng `Option<T>` với `Some(giá_trị)` và `None`.
// - `code.split('-').next()` trả về `Option<&str>`, ép xử lý cả nhánh có và không có dữ liệu.
// - `match` kiểm tra toàn diện mọi nhánh giúp compiler bắt lỗi nếu quên xử lý case.
fn find_code_tag(code: &str) -> Option<&str> {
    code.split('-').next()
}

fn main() {
    // Test Slice với cả String và string literal
    let raw_code = String::from("ORD-2026-001");
    print_prefix(&raw_code, 8);
    print_prefix("RUST-DAY-3", 4);

    // Test Option + match
    match find_code_tag(&raw_code) {
        Some(prefix) => println!("Mã đầu: {}", prefix),
        None => println!("Không tách được mã đầu"),
    }

    // Test Struct + impl + match trạng thái
    let mut order = Order::new(101, &raw_code);
    order.summary();

    order.mark_paid(250_000);
    match &order.status {
        OrderStatus::Pending => println!("Đang chờ thanh toán"),
        OrderStatus::Paid { amount } => println!("Đã thanh toán: {} VND", amount),
        OrderStatus::Cancelled(reason) => println!("Đã hủy vì: {}", reason),
    }

    order.cancel("Khách đổi ý");
    order.summary();
}