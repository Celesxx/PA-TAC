extern crate image;

use image::{open, ImageFormat, DynamicImage, GenericImageView};
use std::os::raw::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn image_to_vector(path: *const c_char) -> *mut u8 
{
    let c_str = unsafe { CStr::from_ptr(path) }; // convertit le pointeur du chemin en type CStr
    let path_str = c_str.to_str().unwrap(); //convertit en chaine de caractère et gère l'échec avec unwrap
    let img = open(path_str).expect("Failed to open image"); // ouvre l'image du chemin
    let gray_img = img.into_luma8();
    // let resized = image::imageops::resize(&gray_img, 200, 200, image::imageops::FilterType::Triangle);
    let vec = gray_img.into_raw(); //convertit l'image en vecteur 
    let boxed_slice = vec.into_boxed_slice();
    let raw = Box::into_raw(boxed_slice) as *mut u8;
    raw
}