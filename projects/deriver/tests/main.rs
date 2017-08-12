use deriver::From;

#[test]
fn ready() {
    println!("it works!")
}


#[derive(From)]
enum Test {
    A(u8),
    B(Option<u8>),
    C(Box<u8>),
}
