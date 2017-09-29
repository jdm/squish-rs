extern crate libc;
use libc::c_int;
use std::os::raw::c_void;

mod ffi {
    use super::*;
    extern "C" {
        pub fn decompress(rgba: *mut u8, block: *const c_void, flags: c_int);
        pub fn DecompressImage(rgba: *mut u8, width: c_int, height: c_int, blocks: *const c_void, flags: c_int);
        pub fn getStorageRequirements(width: c_int, height: c_int, flags: c_int) -> c_int;
    }
}

#[repr(C)]
pub enum CompressType {
    Dxt1 = 1,
    Dxt3 = 2,
    Dxt5 = 4,
}

pub fn decompress(rgba: &mut [u8; 16], block: *const c_void, compress_type: CompressType) {
    unsafe {
        ffi::decompress(rgba.as_mut_ptr(), block, compress_type as c_int);
    }
}

pub fn decompress_image(width: i32, height: i32, blocks: *const c_void, compress_type: CompressType) -> Vec<u8> {
    let size = width as usize * height as usize * 4;
    let mut buffer = Vec::with_capacity(size);
    unsafe {
        ffi::DecompressImage(buffer.as_mut_ptr(), width, height, blocks, compress_type as c_int);
        buffer.set_len(size);
    }
    buffer
}

pub fn get_storage_requirements(width: i32, height: i32, compress_type: CompressType) -> usize {
    unsafe {
        ffi::getStorageRequirements(width as c_int, height as c_int, compress_type as c_int) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(get_storage_requirements(8, 8, CompressType::Dxt3),
                   2 * 2 * 16);
    }
}
