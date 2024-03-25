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