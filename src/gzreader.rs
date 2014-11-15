use std::io::{IoResult, standard_error, EndOfFile, OtherIoError, IoError};
use std::c_str::CString;
use libc::{c_int, c_char, c_uint};


enum GzFile {}

#[link(name="z")]
extern {
    fn gzopen(filename: *const c_char, mode: *const c_char) -> *const GzFile;
    fn gzread(file: *const GzFile, buf: *mut u8, len: c_uint) -> c_int;
    fn gzclose(file: *const GzFile) -> c_int;
    fn gzerror(file: *const GzFile, errnum: *mut c_int) -> *const c_char;
}

pub struct GzipReader {
    file: *const GzFile,
    path: Path,
    closed: bool,
}

impl GzipReader {
    pub fn open(path: &Path) -> IoResult<GzipReader> {
        let file = unsafe {
            gzopen(path.to_c_str().unwrap(), "rb".to_c_str().unwrap())
        };
        if !file.is_null() {
            Ok(GzipReader{file: file, path: path.clone(), closed: false})
        } else {
            Err(IoError::last_error())
        }
    }

    fn get_error(&mut self) -> Option<IoError> {
        let mut errnum: c_int = 0;
        let msg = unsafe {CString::new(gzerror(self.file, &mut errnum), true)};
        if errnum < 0 {
            let detail = self.path.as_str().unwrap().into_string()
                         + msg.as_str().unwrap();
            Some(IoError{kind: OtherIoError,
                         desc: "GZip reader error",
                         detail: Some(detail)
                        })
        } else {
            None
        }
    }

    fn close(&mut self) -> Option<IoError> {
        if !self.closed {
            self.closed = true;
            match unsafe {gzclose(self.file)} {
                0 => None,
                _ => self.get_error()
            }
        } else {
            None
        }
    }
}

impl Drop for GzipReader {
    fn drop(&mut self) {
        self.close();
    }
}

impl Reader for GzipReader {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
        match unsafe {
                    gzread(self.file, buf.as_mut_ptr(), buf.len() as c_uint)
                } {
            size if size > 0 => Ok(size as uint),
            0 => {
                match self.close() {
                    None => Err(standard_error(EndOfFile)),
                    Some(err) => Err(err)
                }
            }
            _ => {
                let err = self.get_error().unwrap();
                self.close();
                Err(err)
            }
        }
    }
}
