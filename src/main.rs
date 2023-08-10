#![no_std]
// #![no_main] // To overwrite the entry point. This fixes error "error: requires `start` lang_item"
use core::panic::PanicInfo;

/// This function is called on panic.
/// Fixes the error "error: `#[panic_handler]` function required, but not found"
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }

// #[no_mangle] // Disables name mangling to ensure the Rust compiler really outputs a function with the name "_start". We'll need this exact name
// pub extern "C" fn _start() -> ! {
//   // The entry point needs to be called _start regardless of your host OS.
//   loop {}
// }

pub fn count_words(val: &str) -> usize {
  val.chars().filter(|c| *c == ' ').count() + 1
}

#[test]
fn test_count_words() {
  assert_eq!(count_words("hello world"), 2);
  assert_eq!(count_words("hello, rusty world"), 3);
}