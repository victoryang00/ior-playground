extern crate libc;

use libc::{c_char, c_int};
use std::ffi::CString;
use anyhow::{Error, Result};
use log::*;
use std::fs::File;
use std::io::{Read, Write};

fn test_write() -> Result<()> {
    let mut f = File::create("/storage/io500-test/testfile")?;
    f.write("123456\n".as_bytes())?;
    Ok(())
}

fn test_read() -> Result<()> {
    let mut f = File::open("/storage/io500-test/testfile")?;
    let mut result = String::new();
    f.read_to_string(&mut result)?;
    println!("{}", result);
    Ok(())
}

extern "C" {
    /// Allocate zero-initialized `size` bytes.
    ///
    /// Returns a pointer to newly allocated zero-initialized memory, or null if
    /// out of memory.
    pub fn ior_main(argc: c_int, argv: *const *const c_char);
}

fn main() -> Result<()> {

    let args = std::env::args().map(|arg| CString::new(arg).unwrap()).collect::<Vec<CString>>();
    let c_args = args.iter().map(|arg | arg.as_ptr()).collect::<Vec<*const c_char>>();
    unsafe {
        ior_main(c_args.len() as c_int, c_args.as_ptr());
    }
    Ok(())
}
