struct SomeType {
    a: u32,
    b: u32,
}

const SOME_CONSTANT: SomeType = SomeType {
    a: 22 + 22,
    b: 44 + 44,
};

fn main() {
    let v: SomeType = SomeType{
        a: 22,
        b: 44,
    };
    match v {
        SOME_CONSTANT => println!("Yes"),
        _ => println!("No"),
    }
}
