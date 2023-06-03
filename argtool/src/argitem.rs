use std::fmt;

use anyhow::anyhow;

/// enum for argtype
/// There are four argument type can be used. The default is [ArgType::BaseType]
pub enum ArgType {
    /// BaseType:
    /// - The classic type
    /// - a flag following by a content
    ///
    /// ```bash
    /// ./program -a "One"
    /// ```
    BaseType,
    /// PositionalType:
    /// - Positional argument with no argument flag specified.
    /// - Parse to specific argument according to position
    /// - Always required
    ///
    /// ```bash
    /// ./program "One"
    /// ```
    PositionalType,

    /// ListType:
    /// - Like the BaseType, but can be specified multiple times.
    /// - Will be paresed as Vec<String>
    ///
    /// ```bash
    /// ./program -a "One" -a "Two"
    /// ```
    ListType,
    /// FlagType
    /// - flag only argument. with no content
    /// - return will always be bool, default is false, required is set to false
    ///
    /// ```bash
    /// ./program -a
    /// ```
    FlagType,
}

/// Display for argtype
#[doc(hidden)]
impl fmt::Display for ArgType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgType::BaseType => write!(f, "Base"),
            ArgType::FlagType => write!(f, "Flag"),
            ArgType::PositionalType => write!(f, "Positional"),
            ArgType::ListType => write!(f, "List"),
        }
    }
}

/// struct for one argument
/// An example
/// ```
/// use argtool::{ArgItem, ArgType};
///
/// let arg = ArgItem::new("line", "n")
///     .set_detail("number of line")
///     .set_default("-1", false)
///     .set_argtype(ArgType::BaseType);
/// ```

pub struct ArgItem {
    pub name: String,
    pub arg_type: ArgType,

    pub alias: Vec<String>,
    pub detail: String,

    pub required: bool,
    pub default: String,
}

/// The default value for argitem
/// - arg_type -> [ArgType::BaseType]
/// - detail   -> "No detail"
/// - required -> true
/// - default -> ""
impl Default for ArgItem {
    fn default() -> ArgItem {
        ArgItem {
            name: String::from(""),
            arg_type: ArgType::BaseType,

            alias: vec![],
            detail: String::from("No detail"),

            required: true,
            default: String::from(""),
        }
    }
}

/// Display detail for an argument
#[doc(hidden)]
impl fmt::Display for ArgItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let ArgType::PositionalType = self.arg_type {
            writeln!(f, " <{}>: {}", self.name, self.detail)?;
            writeln!(
                f,
                "     type: {}, required: {}",
                self.arg_type, self.required
            )?;
        } else {
            writeln!(f, " --{}: {}", self.name, self.detail)?;
            writeln!(
                f,
                "     type: {}, required: {}; default: {}, alias: {:?}",
                self.arg_type, self.required, self.default, self.alias
            )?;
        }
        Ok(())
    }
}

impl<'a> ArgItem {
    /// create a new argument
    pub fn new(name: &'a str, alias: &'a str) -> ArgItem {
        ArgItem {
            name: name.to_string(),
            alias: vec![alias.to_owned()],
            ..Default::default()
        }
    }

    pub fn set_detail(mut self, detail: &'a str) -> Self {
        self.detail = detail.to_string();
        self
    }

    pub fn set_default(mut self, default: &'a str, required: bool) -> Self {
        self.default = default.to_string();
        self.required = required;
        self
    }

    pub fn set_argtype(mut self, arg_type: ArgType) -> Self {
        self.arg_type = arg_type;
        self
    }

    pub fn add_alias(mut self, alias: &'a str) -> Self {
        self.alias.push(alias.to_owned());
        self
    }
}

#[derive(Clone, Debug)]
pub enum ArgValue {
    STR(String),
    VEC(Vec<String>),
}

#[doc(hidden)]
impl fmt::Display for ArgValue {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgValue::STR(v) => write!(f, "{}", v),
            ArgValue::VEC(v) => write!(f, "{:?}", v),
        }
    }
}

impl ArgValue {
    fn match_type(&self) -> Result<String, anyhow::Error> {
        match self {
            ArgValue::STR(v) => Ok(v.clone()),
            _ => Err(anyhow!("Argument is not scalar")),
        }
    }

    pub fn get_bool(&self) -> Result<bool, anyhow::Error> {
        let v = self
            .match_type()?
            .parse::<bool>()
            .unwrap_or_else(|err| panic!("Problem parsing arguments{}", err));
        Ok(v)
    }

    pub fn get_f32(&self) -> Result<f32, anyhow::Error> {
        let v = self
            .match_type()?
            .parse::<f32>()
            .unwrap_or_else(|err| panic!("Problem parsing arguments{}", err));
        Ok(v)
    }

    pub fn get_i32(&self) -> Result<i32, anyhow::Error> {
        let v = self
            .match_type()?
            .parse::<i32>()
            .unwrap_or_else(|err| panic!("Problem parsing arguments{}", err));
        Ok(v)
    }

    pub fn get_string(&self) -> Result<String, anyhow::Error> {
        let v = self.match_type()?;
        Ok(v)
    }

    pub fn get_vec(&self) -> Result<Vec<String>, anyhow::Error> {
        match self {
            ArgValue::VEC(v) => Ok(v.clone()),
            _ => Err(anyhow!("Argument is not vector")),
        }
    }

    #[doc(hidden)]
    pub fn push_vec(&mut self, item: String) -> Result<(), anyhow::Error> {
        match self {
            ArgValue::VEC(v) => v.push(item),
            _ => return Err(anyhow!("Argument is not vector")),
        };
        Ok(())
    }
}
