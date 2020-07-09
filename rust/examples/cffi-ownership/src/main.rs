use std::os::raw::{c_char, c_int, c_void};

#[link(name = "ownership", kind = "static")]
extern "C" {
    fn take_ownership(i: *const c_int, dtor: unsafe extern "C" fn(i: *mut c_int)) -> c_void;
}

#[link(name = "ownership", kind = "static")]
extern "C" {
    fn make_memory() -> *mut c_int;
}

unsafe extern "C" fn drop_pointer(i: *mut c_int) {
    Box::from_raw(i);
}

enum File {}

extern "C" {
    fn fopen(fname: *const c_char, mode: *const c_char) -> *mut File;

    fn fgetc(stream: *mut File) -> c_int;

    fn fclose(stream: *mut File) -> c_int;
}

use libc::{suseconds_t, time_t};
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
struct Timeval {
    tv_sec: time_t,
    tv_usec: suseconds_t,
}

#[repr(C)]
#[derive(Debug)]
struct Timezone {
    tz_minuteswest: c_int,
    tz_dsttime: c_int,
}

extern "C" {
    fn gettimeofday(tv: *mut Timeval, tz: *mut Timezone) -> c_int;
}

fn main() {
    let i = Box::new(1);
    unsafe {
        take_ownership(Box::into_raw(i), drop_pointer);
    }

    unsafe {
        let i = make_memory();

        println!("got {}", *i);

        libc::free(i as *mut _);
    }

    unsafe {
        let fname: *const c_char = b"Cargo.toml\0".as_ptr() as *const _;
        let mode: *const c_char = b"r\0".as_ptr() as *const _;

        let file = fopen(fname, mode);
        if file.is_null() {
            println!("open file failed");
            return;
        }

        loop {
            let c = fgetc(file);
            if c == -1 {
                break;
            } else {
                let c = c as u8 as char;
                print!("{}", c);
            }
        }

        if fclose(file) == -1 {
            println!("close file failed");
        }
    }

    unsafe {
        let mut tv: Timeval = mem::uninitialized();
        let tz: *mut Timezone = ptr::null_mut();
        let ret = gettimeofday(&mut tv as *mut _, tz);
        if ret == -1 {
            println!("failure");
            return;
        }

        println!("{:?}", tv);
    }
}
