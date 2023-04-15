use std::{fmt, slice};
use std::fmt::{Formatter, write};

// Copy 和 Drop 是互斥的
#[derive(Clone, Copy)]
struct RawBuffer {
    ptr: *mut u8,
    len: usize,
}

impl From<Vec<u8>> for RawBuffer {
    fn from(value: Vec<u8>) -> Self {
        let slice = value.into_boxed_slice();
        Self {
            len: slice.len(),
            ptr: Box::into_raw(slice) as *mut u8,
        }
    }
}

impl fmt::Debug for RawBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let data = self.as_ref();
        write!(f, "{:p}: {:?}", self.ptr, data)
    }
}

impl AsRef<[u8]> for RawBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.ptr, self.len)
        }
    }
}

impl Drop for RawBuffer {

    fn drop(&mut self) {
        let data =
    }
}

fn main() {
    let data = vec![1, 2, 3, 4];

    let buf: RawBuffer = data.into();

    use_buffer(buf);

    println!("buf: {:?}", buf);
}

fn use_buffer(buf: RawBuffer) {
    println!("buf to die: {:?}", buf);

    drop(buf)
}