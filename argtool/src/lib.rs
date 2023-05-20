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

pub mod argitem;
pub mod argroup;


pub use argitem::ArgItem;
pub use argitem::ArgType;
pub use argitem::ArgValue;
pub use argroup::ArgGroup;
