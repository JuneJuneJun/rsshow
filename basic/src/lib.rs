
use libloading::{Library, Symbol as LibrarySymbol};

#[cfg(test)]
mod tests {
    extern crate libloading as lib;
    #[test]
    fn it_works() {
        println!("{}", call_dynamic().unwrap());

    }

    fn call_dynamic() -> Result<u32, Box<dyn std::error::Error>> {
        let lib = lib::Library::new("/Users/june/Work/go/src/github.com/tinygo-org/go-llvm/test1.o")?;
        unsafe {
            let func: lib::Symbol<unsafe extern fn() -> u32> = lib.get(b"f0")?;
            Ok(func())
        }
    }


}
