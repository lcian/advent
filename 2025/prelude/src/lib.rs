#![no_std]

use core::arch::asm;

#[macro_export]
macro_rules! prelude {
    () => {
        #[cfg(not(test))] // workaround for rust-analyzer
        mod panic_handler {
            use super::*;
            use core::panic::PanicInfo;

            #[panic_handler]
            fn handle_panic(info: &PanicInfo) -> ! {
                print("panicked in ");
                println(info.location().unwrap().file());
                exit(1)
            }
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn _start() -> ! {
            main();
            exit(0)
        }
    };
}

pub fn exit(code: u32) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") 60,
            in("rdi") code,
        );
    }
    unreachable!();
}

pub fn write(fd: u64, s: &str) {
    unsafe {
        asm!(
            "syscall",
            in("rax") 1,
            in("rdi") fd,
            in("rsi") s.as_ptr(),
            in("rdx") s.len()
        );
    }
}

pub fn atoi(s: &str) -> u64 {
    let mut res = 0;
    for d in s.chars() {
        res = res * 10 + (d as u64 - 48);
    }
    res
}

pub fn itoa(mut x: u64) -> [u8; 20] {
    let mut res: [u8; 20] = [0; 20];
    for i in (0..20).rev() {
        res[i] = 48 + (x % 10) as u8;
        x = x / 10;
    }
    res
}

fn print_inner(s: &str) {
    write(1, s);
}

pub trait Printable {
    fn print(&self);
}

pub fn print<T: Printable>(t: T) {
    t.print();
}

pub fn println<T: Printable>(t: T) {
    t.print();
    print_inner("\n");
}

impl Printable for &str {
    fn print(&self) {
        print_inner(self);
    }
}

macro_rules! printable_unsigned {
    ($T:ty) => {
        impl Printable for $T {
            fn print(&self) {
                let buf = itoa(*self as u64);
                let s = unsafe { str::from_utf8_unchecked(&buf) }.trim_start_matches("0");
                let s = if s.is_empty() { "0" } else { s };
                print_inner(s);
            }
        }
    };
}

macro_rules! printable_signed {
    ($T:ty) => {
        impl Printable for $T {
            fn print(&self) {
                let x = *self;
                if x < 0 {
                    print_inner("-");
                }
                (x.abs() as u64).print();
            }
        }
    };
}

printable_unsigned!(u8);
printable_unsigned!(u16);
printable_unsigned!(u32);
printable_unsigned!(u64);
printable_unsigned!(usize);
printable_signed!(i8);
printable_signed!(i16);
printable_signed!(i32);
printable_signed!(i64);
