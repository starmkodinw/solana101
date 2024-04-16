solana101

- rustc --version
- rustc main.rs <!-- build -->
- `#![feature]` may not be used on the stable release channel -> rm -rf target
- Cargo.toml เหมือน package.json
- cargo --version
- cargo new hello
- cargo build
- cargo run
- cargo bench
- cargo add serde
- serde_json = "1.0.79" <!-- ยัดลงตรงๆใน Cargo.toml -->
- let mut x: i32 = 10; <!-- mut = mutable = เปลี่ยนค่าได้ -->
- Struct เหมือน golang
- impl = implement หมายถึง การนำเสนอ methods, functions หรือ trait ให้ struct
- trait (ลักษณะนิสัย) คล้ายๆ interface, มอง trait เป็น skill
-   
    trait Sayable {
        fn say(&self) -> String;
    }

    // เพิ่ม Sayable skill ให้ Animal
    impl Sayable for Animal {
        fn say(&self) -> String {
            "meow!".to_owned() // convert &str to String.
        }
    }
- serialize = แปลง struct <=> (json, xml, protobuf)
- Crate (เคลท = ลัง) ใน Rust หมายถึง Package หรือ Library ที่ใช้สำหรับแบ่งปันโค้ดกับผู้อื่น
- Borsh เป็น crate ในภาษา Rust ที่ใช้สำหรับ serialization และ deserialization ของข้อมูล
- solana validator node setup
    sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install)"
    solana-install update
    solana --version
    solana config set --url localhost
    solana config set --url devnet
    solana config get
    solana-test-validator
    solana-keygen new -o /Users/tanaponinprasit/.config/solana/id.json
    solana account
    solana address
    solana balance
    <!-- change phantom to local network -->
    solana airdrop 1 EoiXutDVt6wRvq8HWtyDtePbbzaTpVQRRCjTWgPiYLSt
    solana balance EoiXutDVt6wRvq8HWtyDtePbbzaTpVQRRCjTWgPiYLSt
    solana program deploy ./dist/program/helloworld.so
- Ownership rules
    1. value จะมี owner เพียงอันเดียว
        let x = 5; // x เป็นเจ้าของค่า 5
    2. value มี owner ได้เพียงอันเดียวเท่านั้นในเวลาใดเวลาหนึ่ง - เมื่อย้ายข้อมูลไปยังตัวแปรอื่น เจ้าของเดิมจะถูกยกเลิก
    3. เมื่อ owner out of scope จะถูก dropped
        fn main() {
            let x = 5;  // x ถูกประกาศภายในฟังก์ชัน main หลังจากปิดฟังก์ชัน main ตัวแปร x จะถูกเคลียร์
        }
- stack auto clear memory
- heap auto clear memory ด้วยเมื่อ out of scope (บางภาษาไม่ clear ต้องรอ GC มาจัดการ)
- let n1 = Box::new(1) เก็บค่า 1 ลงใน heap, n1 ชี้ไปที่ address ของ heap
- &str เรียกว่า string slices
- String
    stack เก็บ address ของ heap, cap, len
    heap เก็บ ค่าของ string
- Array fixed size
- Collection (vec) ไม่ fixed size
- สร้างพวก module ต่างๆในไฟล์ lib.rs เช่น struct แล้วประกาศแบบ pub เพื่อให้เรียกใช้ได้
- Closures เหมือน anonymouse function
    let x = |a: i32, b: i32| { a+b };
- ? ช่วยในการ throw error ออกไปนอก function แทนที่จะ match, ทำให้โค้ดสั้น อ่านง่ายขึ้น
- Stack เข้าถึงได้เร็ว, หน่วยความจำคงที่ (fixed size) เช่น &str, Array
- Heap เข้าถึงได้ช้า, ขยาย/ลด ขนาดของข้อมูลได้อย่างอิสระ เช่น String, Vec
- Stack ทำงานอย่างไร
    การทำงานของ stack ประกอบไปด้วย 2 ส่วน คือ stack frame และ stack pointer
    - 1. Stack frame คือ พื้นที่ใน stack ที่ OS จองให้กับแต่ละ function ที่ถูกเรียกใช้ในโปรแกรมของเรา โดยขนาดของ stack frame จะมีค่าเท่ากับขนาดของตัวแปรทั้งหมดที่ใช้ใน function และข้อมูลจิปาถะอื่นๆ (return_address) ที่ CPU ต้องการจาก function นั้น พื้นที่ใน stack frame จะถูกจองแบบพอดีเป๊ะกับที่ function ต้องใช้
    - 2. Stack pointer (SP) มีหน้าที่ชี้ไปยัง stack frame ล่าสุดที่ OS เพิ่งจองให้ เนื่องจาก stack frame จะมีการสร้างเพิ่ม หรือ ลบออก ตามการเรียกใช้งาน function อยู่ตลอด จึงต้องมี stack pointer ในการชี้ไปยัง stack frame ล่าสุดที่กำลังทำงานอยู่ เพื่อให้ CPU เก็บข้อมูลได้ง่ายและรวดเร็ว
- return_address เป็น pointer มีขนาด 8 Byte คือ ตำแหน่งของฟังก์ชันก่อนหน้า ที่จะกลับไป หลังจากจบการทำงานของฟังก์ชันปัจจุบัน
- การใช้งาน Heap ด้วย Box
    let a: Box<i32> = Box::new(22); <!-- ค่า 22 ที่ปกติจะเก็บอยู่ใน stack ก็ย้ายไปอยู่ใน heap ทันที -->
- ทำไม Heap ถึงช้ากว่า Stack
    1. ด้านการจองข้อมูล
        heap จะช้ากว่าเนื่องจาก memory allocator จะต้องไล่หาพื้นที่ว่าง
        stack เป็นพื้นที่ที่ถูกระบุบริเวณใน memory allocator มาแต่ต้น
    2. ด้านการเข้าถึงข้อมูล
        heap จะอยู่อย่างกระจัดกระจาย เวลาเข้าถึงค่าต่างๆ ต้องเดินทางข้ามไปมาค่อนข้างไกล
        stack จะอยู่ติดกันเป็นก้อนต่อเนื่อง ระยะทางจึงสั้นกว่า
- dyn = dynamic
- Supertraits
- <T: Learner> = Generic Bounds
- <T: Human + Learner> = Multiple bounds
- () Parentheses
- [] Square Brackets
- {} Curly Braces
- <> Angle Brackets