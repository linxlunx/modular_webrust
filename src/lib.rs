// import rustc_serialize here, so another modules never find problem encodable is not satisfied
extern crate rustc_serialize;

// import mysql for macro_rules params!
extern crate mysql;

// define macro_rules try_with here, so all modules could use try_with macro
#[macro_export]
macro_rules! try_with {
    ($res:expr, $exp:expr) => {{
        match $exp {
            ::std::result::Result::Ok(val) => val,
            ::std::result::Result::Err(e) => {
                return Err(From::from(($res, e)))
            }
        }
    }};
}

// macro use for binding query inside modules folder
#[macro_export]
macro_rules! params {
    ($($name:expr => $value:expr),*) => (
        vec![
            $((::std::string::String::from($name), mysql::Value::from($value))),*
        ]
    );
    ($($name:expr => $value:expr),*,) => (
        params!($($name => $value),*)
    );
}

// list subfolder here
pub mod modules;
pub mod structures;
pub mod helpers;
