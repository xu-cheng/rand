extern crate libc;

use rand_core::{Error, ErrorKind};
use super::OsRngImpl;

use std::io;
use self::libc::{c_int, size_t};

#[derive(Clone, Debug)]
pub struct OsRng;

enum SecRandom {}

#[allow(non_upper_case_globals)]
const kSecRandomDefault: *const SecRandom = 0 as *const SecRandom;

#[link(name = "Security", kind = "framework")]
extern {
    fn SecRandomCopyBytes(rnd: *const SecRandom,
                          count: size_t, bytes: *mut u8) -> c_int;
}

impl OsRngImpl for OsRng {
    fn new() -> Result<OsRng, Error> { Ok(OsRng) }

    fn fill_chunk(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        let ret = unsafe {
            SecRandomCopyBytes(kSecRandomDefault,
                               dest.len() as size_t,
                               dest.as_mut_ptr())
        };
        if ret == -1 {
            Err(Error::with_cause(
                ErrorKind::Unavailable,
                "couldn't generate random bytes",
                io::Error::last_os_error()))
        } else {
            Ok(())
        }
    }

    fn method_str(&self) -> &'static str { "SecRandomCopyBytes" }
}