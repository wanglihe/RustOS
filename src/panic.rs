use core::prelude::*;
use terminal::{TERMINAL, Terminal};
use core::mem::transmute;

pub fn term() -> &'static mut Terminal {
    unsafe {transmute(TERMINAL.deref())}
}

pub fn init() {
  term().clear_screen()
}

pub fn print(string: &'static str) {
  term().print(string);
}

pub fn println(string: &'static str) {
  term().println(string);
}

pub fn panic_message(string: &'static str) -> ! {
  println(string);
  println("^ panic ->");
  loop {}
}

pub fn abort() -> ! {
  panic_message("(from abort)");
}
