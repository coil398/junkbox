use std::cell::RefCell;

struct B {
    c: char,
    s: RefCell<String>,
}

fn main() {
    let b = B {
        c: 'a',
        s: RefCell::new("alex".to_string()),
    };
    let rb = &b;
    rb.s.borrow_mut().push('a');
    {
        let rbs = b.s.borrow();
        assert_eq!(&*rbs, "alexa");

        assert!(b.s.try_borrow_mut().is_err());
    }

    assert!(b.s.try_borrow_mut().is_ok());

    use std::cell::RefCell;
    use std::collections::HashSet;

    thread_local! {
        static RABBITS: RefCell<HashSet<&'static str>> = {
            let rb = ["ロップイヤー", "ダッチ"].iter().cloned().collect();
            RefCell::new(rb)
        }
    };

    RABBITS.with(|rb| {
        assert!(rb.borrow().contains("ロップイヤー"));
        rb.borrow_mut().insert("ネザーランド・ドワーフ");
    });

    std::thread::spawn(|| RABBITS.with(|rb| rb.borrow_mut().insert("ドワーフホト")))
        .join()
        .expect("Thread error");

    RABBITS.with(|rb| {
        assert!(rb.borrow().contains("ネザーランド・ドワーフ"));
        assert!(!rb.borrow().contains("ドワーフホト"));
    });
}
