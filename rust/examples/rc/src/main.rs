#[derive(Copy, Clone, Debug)]
struct Parent(usize, Child, Child);

#[derive(Copy, Clone, Debug)]
struct Child(usize);

fn main() {
    use std::rc::Rc;

    let mut rc1 = Rc::new(Child(1));

    println!("(a) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);
    {
        let rc2 = Rc::clone(&rc1);
        println!(
            "(b) count: {}, rc1: {:?}, rc2: {:?}",
            Rc::strong_count(&rc1),
            rc1,
            rc2
        );
    }
    println!("(c) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    if let Some(child) = Rc::get_mut(&mut rc1) {
        child.0 += 1;
    }
    println!("(d) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    let weak = Rc::downgrade(&rc1);
    println!(
        "(e) count: {}, rc1, {:?}, weak: {:?}",
        Rc::strong_count(&rc1),
        rc1,
        weak,
    );

    if let Some(rc3) = weak.upgrade() {
        println!(
            "(f) count: {}, rc1: {:?}, rc3: {:?}",
            Rc::strong_count(&rc1),
            rc1,
            rc3,
        );
    }

    std::mem::drop(rc1);
    println!("(g) count: 0, weak.upgrade(): {:?}", weak.upgrade());
}
