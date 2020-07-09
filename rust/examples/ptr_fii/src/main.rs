fn main() {
    let x = 1;
    let xptr: *const i32 = &x;

    unsafe {
        let x = *xptr;
    }

    let mut y = 2;
    let yptr: *mut i32 = &mut y;

    unsafe {
        *yptr = 3;
    }

    let z = Box::new(4);
    let zptr: *const i32 = &*z;

    let s: &[u8] = b"abc";
    let sptr: *const u8 = s.as_ptr();

    unsafe {
        let s = std::slice::from_raw_parts(sptr, s.len());
    }

    let boxed = Box::new(true);
    let ptr: *mut bool = Box::into_raw(boxed);

    unsafe {
        let boxed = Box::from_raw(ptr);
    }
}
