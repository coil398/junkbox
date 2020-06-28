use std::string::ToString;

fn stringify<T: ToString>(t: T) -> String {
    t.to_string()
}

fn stringify(t: Box<dyn ToString>) -> String {
    t.to_string()
}

fn main() {
    stringify(1i32);
    stringify::<i32>(1i32);

    use std::fmt::Display;
    // let mut v: Vec<Display> = vec![];
    // v.push(true);
    // v.push(1i32);

    let mut v: Vec<&dyn Display> = vec![];
    v.push(&true);
    v.push(&1i32);
}
