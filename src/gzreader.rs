use std::path::Path;
use std::io::Reader;
use std::option::{Some, None, Option};
use std::c_str::CString;
use std::os::last_os_error;
use std::ptr::is_null;
use std::libc::types::os::arch::c95::{c_char, c_int, size_t};


enum gzFile {}

#[link(name="z")]
extern {
    fn gzopen(filename: *c_char, mode: *c_char) -> *gzFile;
    fn gzread(file: *gzFile, buf: *mut u8, len: size_t) -> c_int;
    fn gzclose(file: *gzFile) -> c_int;
    fn gzerror(file: *gzFile, errnum: *c_int) -> *c_char;
}

pub struct GzipReader {
    priv file: *gzFile,
    priv eof: bool,
}

impl GzipReader {
    pub fn open(path: &Path) -> Option<GzipReader> {
        let file = unsafe {
            gzopen(path.to_c_str().unwrap(), "rb".to_c_str().unwrap())
        };
        if is_null(file) {
            fail!("ERROR opening '{}': {}", path.as_str(), last_os_error());
        }
        Some(GzipReader{file: file, eof: false})
    }
}

impl Reader for GzipReader {
    fn read(&mut self, buf: &mut [u8]) -> Option<uint> {
        if self.eof {
            return None;
        }
        unsafe {
            match gzread(self.file, buf.as_mut_ptr(), buf.len() as size_t) {
                size if size > 0 => Some(size as uint),
                0 => {
                    // TODO: We can define constants for all the error values
                    // and check gzclose() errors
                    self.eof = true;
                    gzclose(self.file);
                    None
                }
                _ => {
                    let errnum: c_int = 0;
                    let s = CString::new(gzerror(self.file, &errnum), true);
                    // We set EOF here because we close the file
                    self.eof = true;
                    // TODO: We can define constants for all the error values
                    // and check gzclose() errors
                    gzclose(self.file);
                    fail!("ERROR reading file: {}", s.as_str().unwrap());
                }
            }
        }
    }

    fn eof(&mut self) -> bool {
        self.eof
    }
}
