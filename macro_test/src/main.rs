#[cfg(test)]
mod tests {
    use crate::{Pancakes, HelloMacro};

    #[test]
    fn it_works() {
        Pancakes::hello_macro();
    }
}

use hello_macro_derive::HMacro;

pub trait HelloMacro {
    fn hello_macro();
}

// use hello_macro::HelloMacro;
#[derive(HMacro)]
struct Pancakes;


extern "C" {
    fn storage_write(key: &[u8], value: &[u8]);
    fn fvm_storage_write(key: *const u8, klen: u32, val: *const u8, vlen: u32);
}

fn main() {
    // Pancakes::hello_macro();
    let key = &[30,31,32];
    let value = &[60,61,62];
    unsafe {
        storage_write(key, value);
        fvm_storage_write(key.as_ptr(), key.len() as u32, value.as_ptr(), value.len() as u32);
    }
}