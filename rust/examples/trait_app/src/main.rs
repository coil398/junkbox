fn take_string(s: impl Into<String>) {
    let _s = s.into();
}

trait SomeTrait {
    fn take_ref(&self);
}

impl SomeTrait for str {
    fn take_ref(&self) {}
}

#[derive(Debug)]
enum Either<A, B> {
    A(A),
    B(B),
}

use std::fmt;
impl<A, B> fmt::Display for Either<A, B>
where
    A: fmt::Display,
    B: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Either::A(a) => a.fmt(f),
            Either::B(b) => b.fmt(f),
        }
    }
}

fn main() {
    take_string("some_string");
    let arg = "string".to_string();
    take_string(arg.as_str());

    let s = "hoge";
    s.take_ref();

    let box_s = Box::new(s);
    box_s.take_ref();

    let mut v: Vec<Either<bool, i32>> = vec![];
    v.push(Either::A(true));
    v.push(Either::B(1i32));

    for e in v {
        println!("{}", e);
    }
}
