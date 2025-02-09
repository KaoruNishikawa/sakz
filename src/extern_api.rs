use crate::Figure;
use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn sakz_figure_new<'a>(title: *const c_char) -> *mut Figure<'a> {
    let title = unsafe { CStr::from_ptr(title) };
    let title = title.to_str().unwrap();
    let figure = Box::new(Figure::new(title));
    Box::into_raw(figure)
}

#[no_mangle]
pub extern "C" fn sakz_figure_title(figure: *mut Figure, title: *const c_char) -> *mut Figure {
    let title = unsafe { CStr::from_ptr(title) };
    let title = title.to_str().unwrap();
    let figure = unsafe { &mut *figure };
    figure.title(title)
}

#[no_mangle]
pub extern "C" fn sakz_figure_save(figure: *mut Figure, filename: *const c_char) -> i8 {
    let filename = unsafe { CStr::from_ptr(filename) };
    let filename = filename.to_str().unwrap();
    let figure = unsafe { &mut *figure };
    match figure.save(filename) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}
