use ffi::{unqlite_array_count, unqlite_array_walk, unqlite_value, unqlite_value_is_bool,
          unqlite_value_is_float, unqlite_value_is_int, unqlite_value_is_json_array,
          unqlite_value_is_json_object, unqlite_value_is_null, unqlite_value_is_string,
          unqlite_value_to_bool, unqlite_value_to_double, unqlite_value_to_string,
          unqlite_value_to_int64};
use std::collections::HashMap;
use std::os::raw::{c_int, c_void};
use std::slice;
use vars::{UNQLITE_ABORT, UNQLITE_OK};

/// Map of Values
pub type Map = HashMap<String, Value>;

/// Represent all values that can be stored in a Jx9 variable. It is similar to JSON value.
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    Int(i64),
    Bool(bool),
    Real(f64),
    String(String),
    Array(Vec<Value>),
    Object(self::Map),
}

impl Value {
    pub fn string<T: AsRef<str>>(x: T) -> Self {
        Value::String(String::from(x.as_ref()))
    }

    pub fn is_scalar(&self) -> bool {
        match *self {
            Value::Array(_) | Value::Object(_) => false,
            _ => true,
        }
    }
}

macro_rules! declare_converter_to_ex {
    ($march_type: pat, $some_x: expr, $to_type_rs: ty) => (
        impl From<Value> for Option<$to_type_rs> {
            fn from(val: Value) -> Self {
                match val {
                    $march_type => Some($some_x),
                    _ => None,
                }
            }
        }
    )
}

macro_rules! declare_converter_to {
    ($type: path, $to: ty) => (declare_converter_to_ex!($type(x), x, $to);)
}

declare_converter_to_ex!(Value::Null, (), ());
declare_converter_to!(Value::Int, i64);
declare_converter_to!(Value::Real, f64);
declare_converter_to!(Value::Bool, bool);
declare_converter_to!(Value::String, String);
declare_converter_to!(Value::Array, Vec<Value>);
declare_converter_to!(Value::Object, Map);

/// Extract Jx9 value and copy to Value object
pub unsafe fn to_value(ptr: *mut unqlite_value) -> Option<Value> {
    const TRUE: i32 = 1;
    if ptr.is_null() {
        None
    } else if unqlite_value_is_null(ptr) == TRUE {
        Some(Value::Null)
    } else if unqlite_value_is_bool(ptr) == TRUE {
        Some(Value::Bool(unqlite_value_to_bool(ptr) != 0))
    } else if unqlite_value_is_int(ptr) == TRUE {
        Some(Value::Int(unqlite_value_to_int64(ptr) as i64))
    } else if unqlite_value_is_float(ptr) == TRUE {
        Some(Value::Real(unqlite_value_to_double(ptr) as f64))
    } else if unqlite_value_is_string(ptr) == TRUE {
        let string = value_to_string(ptr);
        debug_assert!(string.is_some());
        string.map(Value::String)
    } else if unqlite_value_is_json_object(ptr) == TRUE {
        let size: c_int = unqlite_array_count(ptr);
        debug_assert!(size >= 0);
        if size < 0 {
            None
        } else {
            let map: Box<Map> = Box::new(Map::with_capacity(size as usize));
            let map = Box::into_raw(map);
            let walk_result: c_int = unqlite_array_walk(ptr, Some(map_walk), map as *mut c_void);
            debug_assert_eq!(UNQLITE_OK, walk_result);
            if walk_result != UNQLITE_OK {
                None
            } else {
                Some(Value::Object(*Box::from_raw(map)))
            }
        }
    } else if unqlite_value_is_json_array(ptr) == TRUE {
        let size: c_int = unqlite_array_count(ptr);
        debug_assert!(size >= 0);
        if size < 0 {
            None
        } else {
            let values: Box<Vec<Value>> = Box::new(Vec::with_capacity(size as usize));
            let values = Box::into_raw(values);
            let walk_result: c_int =
                unqlite_array_walk(ptr, Some(array_walk), values as *mut c_void);
            debug_assert_eq!(UNQLITE_OK, walk_result);
            if walk_result != UNQLITE_OK {
                None
            } else {
                Some(Value::Array(*Box::from_raw(values)))
            }
        }
    } else {
        unreachable!("Failed to detect unqlite_value type")
    }
}

unsafe fn value_to_string(ptr: *mut unqlite_value) -> Option<String> {
    let mut len: c_int = 0;
    let cstr: *const i8 = unqlite_value_to_string(ptr, &mut len);
    if cstr.is_null() {
        None
    } else {
        let slice: &[u8] = slice::from_raw_parts(cstr as *const u8, len as usize);
        let string = String::from_utf8_lossy(slice).to_string(); // clone bytes
        Some(string)
    }
}

unsafe extern "C" fn array_walk(
    _: *mut unqlite_value,
    value: *mut unqlite_value,
    collection: *mut c_void,
) -> i32 {
    let collection = collection as *mut Vec<Value>;
    match to_value(value) {
        Some(value) => {
            (*collection).push(value);
            UNQLITE_OK
        }
        None => UNQLITE_ABORT,
    }
}

unsafe extern "C" fn map_walk(
    key: *mut unqlite_value,
    value: *mut unqlite_value,
    collection: *mut c_void,
) -> i32 {
    let collection = collection as *mut Map;
    let both = value_to_string(key).and_then(|k| to_value(value).map(|v| (k, v)));
    match both {
        Some((k, v)) => {
            (*collection).insert(k, v);
            UNQLITE_OK as _
        }
        None => UNQLITE_ABORT,
    }
}
