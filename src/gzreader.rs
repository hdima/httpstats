use std::path::Path;
use std::io::{Reader, IoResult, standard_error, EndOfFile, OtherIoError};
use std::option::{Some, Option};
//use std::c_str::CString;
use std::os::last_os_error;
use libc::types::os::arch::c95::{c_char, c_int, size_t};


enum GzFile {}

#[link(name="z")]
extern {
    fn gzopen(filename: *const c_char, mode: *const c_char) -> *const GzFile;
    fn gzread(file: *const GzFile, buf: *mut u8, len: size_t) -> c_int;
    fn gzclose(file: *const GzFile) -> c_int;
    //fn gzerror(file: *GzFile, errnum: *c_int) -> *c_char;
}

pub struct GzipReader {
    file: *const GzFile,
}

impl GzipReader {
    pub fn open(path: &Path) -> Option<GzipReader> {
        let file = unsafe {
            gzopen(path.to_c_str().unwrap(), "rb".to_c_str().unwrap())
        };
        if file.is_null() {
            fail!("ERROR opening '{}': {}", path.as_str(), last_os_error());
        }
        Some(GzipReader{file: file})
    }
}

impl Reader for GzipReader {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
        unsafe {
            match gzread(self.file, buf.as_mut_ptr(), buf.len() as size_t) {
                size if size > 0 => Ok(size as uint),
                0 => {
                    // TODO: We can define constants for all the error values
                    // and check gzclose() errors
                    gzclose(self.file);
                    Err(standard_error(EndOfFile))
                }
                _ => {
                    // FIXME: See below
                    //let errnum: c_int = 0;
                    //let s = CString::new(gzerror(self.file, &errnum), true);
                    // TODO: We can define constants for all the error values
                    // and check gzclose() errors
                    // FIXME: See above
                    //fail!("ERROR reading file: {}", s.as_str().unwrap());
                    gzclose(self.file);
                    Err(standard_error(OtherIoError))
                }
            }
        }
    }
}
