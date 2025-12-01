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

pub fn print(s: &str) {
    write(1, s);
}

pub fn println(s: &str) {
    print(s);
    print("\n");
}

pub fn println_int(x: u64) {
    let x = itoa(x as u64);
    let s = str::from_utf8(&x).unwrap().trim_start_matches('0');
    let s = if s.is_empty() { "0" } else { s };
    println(s);
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
