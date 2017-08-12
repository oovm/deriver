use deriver::From;

#[test]
fn ready() {
    println!("it works!")
}

#[derive(From)]
enum Test {
    A1(u8),
    // #[from(ignore)]
    // A2(u8),
    B(Option<u16>),
    C(Box<u32>),
}
