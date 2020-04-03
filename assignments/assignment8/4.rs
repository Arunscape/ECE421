#![feature(asm)]
fn main() {
    let message = String::from("James, you are completely mad\n");
    syscall(message);
}

#[cfg(target_os = "linux")]
fn syscall(message: String) {
    let msg_ptr = message.as_ptr();
    let len = message.len();
    unsafe {
        asm!("
             mov $$1, %rax #
             mov $$1, %rdi #
             mov $0, %rsi  #
             mov $1, %rdx  #
             syscall       #
             "
                :
                : "r"(msg_ptr), "r"(len))}}
