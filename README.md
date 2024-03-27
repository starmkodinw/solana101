solana101

- rustc --version
- rustc main.rs <!-- build -->
- Cargo.toml เหมือน package.json
- cargo new hello
- cargo build
- cargo run
- cargo add serde
- serde_json = "1.0.79" <!-- ยัดลงตรงๆใน Cargo.toml -->
- let mut x: i32 = 10; <!-- mut = mutable = เปลี่ยนค่าได้ -->
- Struct เหมือน golang
- impl = implement หมายถึง การนำเสนอ methods, functions หรือ trait ให้ struct
- trait คล้ายๆ interface
- serialize = แปลง struct <=> (json, xml, protobuf)
- Crate (เคลท = ลัง) ใน Rust หมายถึง Package หรือ Library ที่ใช้สำหรับแบ่งปันโค้ดกับผู้อื่น
- Borsh เป็น crate ในภาษา Rust ที่ใช้สำหรับ serialization และ deserialization ของข้อมูล
- solana validator node setup
    sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install)"
    solana-install update
    solana --version
    solana config set --url localhost
    solana-test-validator
    solana-keygen new -o /Users/tanaponinprasit/.config/solana/id.json
    solana account
    solana address
    solana balance
    <!-- change phantom to local network -->
    solana airdrop 1 EoiXutDVt6wRvq8HWtyDtePbbzaTpVQRRCjTWgPiYLSt
    solana balance EoiXutDVt6wRvq8HWtyDtePbbzaTpVQRRCjTWgPiYLSt
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