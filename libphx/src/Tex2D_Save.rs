use crate::internal::ffi;
use crate::Common::*;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat};

#[no_mangle]
pub extern "C" fn Tex2D_Save_Png(
    path: *const libc::c_char,
    sx: i32,
    sy: i32,
    components: i32,
    data: *mut u8,
) -> bool {
    let buffer =
        unsafe { std::slice::from_raw_parts(data, (sx * sy * components) as usize) }.to_vec();
    let img: DynamicImage = match components {
        3 => DynamicImage::ImageRgb8(ImageBuffer::from_raw(sx as u32, sy as u32, buffer).unwrap()),
        4 => DynamicImage::ImageRgba8(ImageBuffer::from_raw(sx as u32, sy as u32, buffer).unwrap()),
        _ => CFatal!(
            "Tex2D_Save_Png: Unexpected number of components %d",
            components
        ),
    };
    img.save(ffi::PtrAsSlice(path)).is_ok()
}
