use crate::wasm4;
use nanoserde::{DeBin, DeBinErr, SerBin};

pub trait Saveable {
    fn save(&self)
    where
        Self: SerBin,
    {
        let bytes = &SerBin::serialize_bin(self)[..];
        let size = (core::mem::size_of::<u8>() * bytes.len()) as u32;
        unsafe {
            wasm4::diskw(bytes.as_ptr(), size);
        }
    }

    fn restore() -> Result<Self, DeBinErr>
    where
        Self: DeBin,
    {
        let mut buffer = [0u8; 1024];
        unsafe {
            wasm4::diskr(
                buffer.as_mut_ptr(),
                (1024 * core::mem::size_of::<u8>()) as u32,
            );
        }
        DeBin::deserialize_bin(&buffer)
    }
}
