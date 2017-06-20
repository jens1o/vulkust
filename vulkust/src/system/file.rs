use std::fs::File as StdFile;
use std::io::{BufReader, Read, Seek, SeekFrom, Result};
use std::mem::{transmute, size_of};

#[derive(Debug)]
pub struct File {
    pub endian_compatible: bool,
    pub reader: BufReader<StdFile>,
}

impl File {
    #[cfg(target_endian = "big")]
    fn check_endian(&mut self) {
        if self.read_bool() {
            self.endian_compatible = false;
        } else {
            self.endian_compatible = true;
        }
    }

    #[cfg(target_endian = "little")]
    fn check_endian(&mut self) {
        if self.read_bool() {
            self.endian_compatible = true;
        } else {
            self.endian_compatible = false;
        }
    }

    pub fn new(file_name: &String) -> Self {
        match StdFile::open(file_name) {
            Ok(f) => {
                let mut s = File {
                    endian_compatible: true,
                    reader: BufReader::new(f)
                };
                s.check_endian();
                s
            }
            Err(e) => {
                logf!("Error {:?} in file reading.", e);
            }
        }
    }

    pub fn read_typed_bytes(&mut self, des: *mut u8, count: usize) {
        let b = self.read_bytes(count);
        if self.endian_compatible {
            for i in 0..count {
                unsafe {
                    *des.offset(i as isize) = b[i];
                }
            }
        } else {
            let mut i = 0isize;
            let mut j = count - 1;
            let count = count as isize;
            while i < count {
                unsafe {
                    *des.offset(i) = b[j];
                }
                i += 1;
                j -= 1;
            }
        }
    }

    pub fn read_bytes(&mut self, count: usize) -> Vec<u8> {
        let mut b = vec![0u8; count];
        let mut read_count = 0;
        while read_count < count {
            let tmp_count = match self.read(&mut b[read_count..count]) {
                Ok(c) => { c },
                Err(_) => { logf!("Error in reading stream."); },
            };
            read_count += tmp_count;
            if tmp_count == 0 {
                logf!(
                    "Expected bytes count is {} but the read bytes count is {}.",
                    count, read_count);
            }
        }
        return b;
    }

    pub fn read_bool(&mut self) -> bool {
        let b = self.read_bytes(1);
        if b[0] == 1 {
            return true;
        }
        return false;
    }

    pub fn read_type<T>(&mut self) -> T where T: Default {
        let mut r = T::default();
        self.read_typed_bytes(unsafe {transmute(&mut r)}, size_of::<T>());
        r
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reader.read(buf)
    }
}

impl Seek for File {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.reader.seek(pos)
    }
}