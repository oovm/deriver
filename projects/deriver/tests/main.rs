use deriver::From;

#[test]
fn ready() {
    println!("it works!")
}


#[derive(From)]
enum Test {
    A(String),
    B(Option<String>),
    C(Box<String>),
}
