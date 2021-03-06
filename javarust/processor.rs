#![crate_type = "dylib"]
use std::os::raw::c_int;

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn processInt(value: i32) -> i32 {
    value.pow(8) / 5 / 2 / 5
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn processFloat(value: f32) -> f32 {
    value.powi(4) * 5f32 / 304.5f32
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn vecLen(vec: *const Vec<c_int>) -> c_int {
    if vec.is_null() {
        return 0;
    }

    (&*vec).len() as c_int
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn vecSumEven(vec: *const c_int, size: i32) -> c_int {
    if vec.is_null() {
        return 0;
    }

    let v =  std::slice::from_raw_parts(vec, size as usize);
    v.iter().filter(|x| *x % &2 == 0).sum::<i32>() as c_int
}

#[repr(C)]
#[no_mangle]
#[derive(Debug)]
pub struct Value {
    a: i32,
    b: i32,
    c: i32,
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn vecMean(vec: *const Value, size: i32) -> f32 {
    if vec.is_null() {
        return 0f32;
    }

    let v =  std::slice::from_raw_parts(vec, size as usize);

    (*v).iter().map(|x| (x.a + x.b + x.c) as f32 / 3f32).sum::<f32>() / size as f32
}