use std::convert::AsRef;
use std::ffi::{CStr, CString};
use std::fmt::Debug;
use std::os::raw::c_char;
use std::slice;

use log::trace;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not convert Rust string {value:?} to C string")]
    StringEncoding {
        value: String,
        source: std::ffi::NulError,
    },
    #[error("Could not convert C string {value:?} to Rust string")]
    StringDecoding {
        value: CString,
        source: std::str::Utf8Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

mod sys {
    #![allow(non_camel_case_types)]
    include!(concat!(env!("OUT_DIR"), "/user_defaults.rs"));
}

fn log_fetching(type_: &str, key: &str) {
    trace!("Fetching {} value of user defaults key {:?}", type_, key);
}

fn log_null(type_: &str, key: &str) {
    trace!(
        "User defaults key {:?} was not present or not a {} value",
        key,
        type_
    );
}

fn log_fetched<V: Debug>(type_: &str, key: &str, value: V) {
    trace!(
        "Fetched {} value {:?} of user defaults key {:?}",
        type_,
        value,
        key
    );
}

fn log_setting<V: Debug>(type_: &str, key: &str, value: V) {
    trace!(
        "Setting user defaults key {:?} to {} value {:?}",
        key,
        type_,
        value
    );
}

fn log_set<V: Debug>(type_: &str, key: &str, value: V) {
    trace!(
        "Set user defaults key {:?} to {} value {:?}",
        key,
        type_,
        value
    );
}

const I64: &str = "i64";

pub fn get_i64(key: &str) -> Result<Option<i64>> {
    log_fetching(I64, key);
    let c_key = to_cstring(&key)?;
    let wrapper = unsafe { sys::user_defaults_get_long(c_key.as_ptr()) };
    if !wrapper.present {
        log_null(I64, key);
        return Ok(None);
    }
    let value = wrapper.value;
    log_fetched(I64, key, value);
    return Ok(Some(value));
}

pub fn set_i64(key: &str, value: i64) -> Result<()> {
    log_setting(I64, key, value);
    let c_key = to_cstring(&key)?;
    unsafe {
        sys::user_defaults_set_long(c_key.as_ptr(), value);
    }
    log_set(I64, key, value);
    Ok(())
}

const F64: &str = "f64";

pub fn get_f64(key: &str) -> Result<Option<f64>> {
    log_fetching(F64, key);
    let c_key = to_cstring(&key)?;
    let wrapper = unsafe { sys::user_defaults_get_double(c_key.as_ptr()) };
    if !wrapper.present {
        log_null(F64, key);
        return Ok(None);
    }
    let value = wrapper.value;
    log_fetched(F64, key, value);
    return Ok(Some(value));
}

pub fn set_f64(key: &str, value: f64) -> Result<()> {
    log_setting(F64, key, value);
    let c_key = to_cstring(&key)?;
    unsafe {
        sys::user_defaults_set_double(c_key.as_ptr(), value);
    }
    log_set(F64, key, value);
    Ok(())
}

const STRING: &str = "string";

pub fn get_string(key: &str) -> Result<Option<String>> {
    log_fetching(STRING, key);
    let c_key = to_cstring(&key)?;
    let value_ptr = unsafe { sys::user_defaults_get_string(c_key.as_ptr()) };
    if value_ptr.is_null() {
        log_null(STRING, key);
        return Ok(None);
    }
    // https://stackoverflow.com/a/24148033/879885
    let value_c_str = unsafe { CStr::from_ptr(value_ptr) };
    let value = to_rust_str(value_c_str)?.to_owned();
    unsafe {
        sys::user_defaults_free_string(value_ptr);
    }
    log_fetched(STRING, key, &value);
    return Ok(Some(value));
}

pub fn set_string(key: &str, value: &str) -> Result<()> {
    log_setting(STRING, key, value);
    let c_key = to_cstring(&key)?;
    let c_value = to_cstring(&value)?;
    unsafe {
        sys::user_defaults_set_string(c_key.as_ptr(), c_value.as_ptr());
    }
    log_set(STRING, key, value);
    Ok(())
}

const STRING_ARRAY: &str = "string array";

pub fn get_string_array(key: &str) -> Result<Option<Vec<String>>> {
    log_fetching(STRING_ARRAY, key);
    let c_key = to_cstring(&key)?;
    let struct_ptr = unsafe { sys::user_defaults_get_string_array(c_key.as_ptr()) };
    if struct_ptr.is_null() {
        log_null(STRING_ARRAY, key);
        return Ok(None);
    }

    let count = unsafe { *struct_ptr }.count;
    let c_values = unsafe {
        slice::from_raw_parts(
            (*struct_ptr).data,
            count as usize, // TODO Every example I see does this cast, but is it safe?
        )
    };
    let mut values = Vec::new();
    for c_value in c_values {
        values.push(to_rust_str(unsafe { CStr::from_ptr(*c_value) })?.to_owned());
    }
    unsafe {
        sys::user_defaults_free_string_array(struct_ptr);
    }
    log_fetched(STRING_ARRAY, key, &values);
    return Ok(Some(values));
}

// https://stackoverflow.com/questions/41179659/convert-vecstring-into-a-slice-of-str-in-rust
// https://users.rust-lang.org/t/vec-string-to-str/12619/4
// https://users.rust-lang.org/t/idiomatic-string-parmeter-types-str-vs-asref-str-vs-into-string/7934
pub fn set_string_array<T: AsRef<str> + std::fmt::Debug>(key: &str, values: &[T]) -> Result<()> {
    log_setting(STRING_ARRAY, key, values);
    let c_key = to_cstring(&key)?;

    let mut c_string_values = Vec::new();
    for value in values {
        c_string_values.push(to_cstring(value.as_ref())?);
    }

    let c_ptr_values: Vec<*const c_char> = c_string_values.iter().map(|s| s.as_ptr()).collect();
    let string_array_struct = sys::user_defaults_string_array {
        count: values.len() as sys::size_t,
        data: c_ptr_values.as_ptr(),
    };
    unsafe {
        sys::user_defaults_set_string_array(c_key.as_ptr(), string_array_struct);
    }
    log_set(STRING_ARRAY, key, values);
    Ok(())
}

pub fn to_cstring(value: &str) -> Result<CString> {
    trace!("Converting Rust string {:?} to C string", value);
    CString::new(value).map_err(|source| Error::StringEncoding {
        value: value.to_owned(),
        source,
    })
}

pub fn to_rust_str(value: &CStr) -> Result<&str> {
    trace!("Converting C string {:?} to Rust string", value);
    value.to_str().map_err(|source| Error::StringDecoding {
        value: value.to_owned(),
        source,
    })
}
