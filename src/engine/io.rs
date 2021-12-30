use crate::wasm4;
use nanoserde::{DeBin, DeBinErr, SerBin};

pub trait Saveable {
    fn save(&self)
    where
        Self: SerBin + Sized,
    {
        let data = SerBin::serialize_bin(self);
        let data_size = (core::mem::size_of::<u8>() * data.len()) as u32;
        let mut to_write = SerBin::serialize_bin(&data_size);
        to_write.append(&mut SerBin::serialize_bin(&data));
        unsafe {
            wasm4::diskw(to_write.as_ptr(), data_size);
        }
    }

    fn restore() -> Result<Self, DeBinErr>
    where
        Self: DeBin,
    {
        let mut size_buffer = [0u8; 4];
        unsafe {
            wasm4::diskr(
                size_buffer.as_mut_ptr(),
                4 * core::mem::size_of::<u8>() as u32,
            );
        }
        let size: u32 = u32::from_le_bytes(size_buffer);
        //TODO: make this better than random hardcoded value?
        let mut bytes_buffer = [0u8; 512];
        unsafe {
            wasm4::diskr(
                bytes_buffer.as_mut_ptr(),
                ((4 + size as usize) * core::mem::size_of::<u8>()) as u32,
            );
        }
        DeBin::deserialize_bin(&bytes_buffer[4..])
    }
}
