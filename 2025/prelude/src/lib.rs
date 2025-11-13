#![no_std]

use core::arch::asm;

#[macro_export]
macro_rules! setup {
    () => {
        #[cfg(not(test))] // workaround for rust-analyzer
        mod panic_handler {
            use super::*;
            use core::panic::PanicInfo;

            #[panic_handler]
            fn handle_panic(info: &PanicInfo) -> ! {
                print("panicked in ".as_bytes());
                println(info.location().unwrap().file().as_bytes());
                exit(1);
            }
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn _start() -> ! {
            main();
            exit(0);
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

pub fn write(fd: u64, slice: &[u8]) {
    unsafe {
        asm!(
            "syscall",
            in("rax") 1,
            in("rdi") fd,
            in("rsi") slice.as_ptr(),
            in("rdx") slice.len()
        );
    }
}

pub fn print(slice: &[u8]) {
    write(1, slice);
}

pub fn println(slice: &[u8]) {
    print(slice);
    print(b"\n");
}
