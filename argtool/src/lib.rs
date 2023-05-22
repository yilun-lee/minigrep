//! Simple and functional argparse
//!
//! Provides an argparser with several argument type ( argtype )
//! there are four argtype:
//! - Basic argtype 
//! - List argtype
//! - Positional argtype
//! - Flag argtype
//! 
//! This module is part of my minigrep, WIP
//! Not opitmized yet

//! # Example
//! ```
//! use argtool::ArgItem;
//! use argtool::ArgGroup;
//! use argtool::ArgType;
//! 
//! // create argument     
//! let name = String::from("Test");
//! let desciption = String::from("This is a Test");
//! let mut my_arg_table: ArgGroup = ArgGroup::new(name, desciption);
//! 
//! // add arg
//! my_arg_table.add_arg(
//!    ArgItem::new("line", "n")
//!    .set_detail("number of line")
//!    .set_default("-1", false)
//!    .set_argtype(ArgType::BaseType)
//!     ).unwrap();
//! 
//! my_arg_table.add_arg(
//!     ArgItem::new("ignorecase", "i")
//!     .set_detail("ignorecase or not ")
//!     .set_default("false", false)
//!     .set_argtype(ArgType::FlagType)
//!     ).unwrap();
//! 
//! // you can print it as help page
//! println!("{}", my_arg_table);
//! 
//! // define mock input
//! let mystr: String = String::from("/lib.rs -n 3 -i ");
//! let my_cmd_iter = mystr.split(" ").into_iter();
//! 
//! // parse
//! let my_arg = my_arg_table.parse(my_cmd_iter).unwrap();
//! println!("{:#?}", my_arg);
//! 
//! // get value 
//! assert_eq!(true, my_arg["ignorecase"].get_bool().unwrap());
//! assert_eq!(3, my_arg_table.get_i32("line").unwrap());
//! ```
//! 
//! # Arg Type
//! - Basic argtype 
//! ```bash
//! ./lib.rs -a "One"
//! ```
//! - List argtype
//! ```bash
//! ./lib.rs -a "One" -a "Two"
//! ```
//! - Positional argtype
//! ```bash
//! ./lib.rs "One"
//! ```
//! - Flag argtype
//! ```bash
//! ./lib.rs -a 
//! ```
//! For detail please check [ArgType](argitem::ArgType)
//! 
//! # Value Type
//! All value are parsed into string in default.
//! In order to parse to vec and scaler in rust, we use an Enum [ArgValue](argitem::ArgValue) to stored value [STR](argitem::ArgValue::STR) and [STR](argitem::ArgValue::VEC).
//! You can parse it as different type according to the parse function used. (e.g. get_bool, get_vec)
//! They will be parse only when you want to get them out of [ArgGroup](argroup::ArgGroup).
//! 
//! There are two method to extract value
//! 1. use [ArgGroup](argroup::ArgGroup) object
//! ```no_test
//! assert_eq!(3, my_arg_table.get_i32("line").unwrap());
//! ```
//! 
//! 2. Use HashMap return from the ArgGroup.parse
//! ```no_test
//! assert_eq!(true, my_arg["ignorecase"].get_bool().unwrap());
//! ```
//! 
//! For detail please check [ArgValue](argitem::ArgValue)
//! 
//! Supported type:
//! - String
//! - i32
//! - f32
//! - bool 
//! - Vec\<String\> (For List Argument only)
//! 

pub mod argitem;
pub mod argroup;

pub use argitem::ArgItem;
pub use argitem::ArgType;
pub use argitem::ArgValue;
pub use argroup::ArgGroup;
