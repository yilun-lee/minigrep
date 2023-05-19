use std::fmt;

use anyhow::anyhow;

pub enum ArgType {
    PositionalType,    // YOUR_PROGRAM  "One"
    BaseType,     // YOUR_PROGRAM -a "One"
    ListType,     // YOUR_PROGRAM -a "One" -a "Two"
    FlagType,     // YOUR_PROGRAM -a
}



pub struct ArgItem {
    pub name: String,
    pub arg_type: ArgType,

    pub alias: Vec<String>,
    pub detail: String,

    pub required: bool,
    pub default: String,
}


impl  Default for ArgItem {
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


impl <'a> ArgItem {
    
    pub fn new(name: &'a str, alias: &'a str) -> ArgItem  {
        ArgItem{
            name: name.to_string(),
            alias: vec![alias.to_owned(),],
            ..Default::default()
        }
    }

    pub fn set_detail(mut self, detail: &'a str) -> Self{
        self.detail = detail.to_string();
        self
    }

    pub fn set_default(mut self, default: &'a str, required: bool) -> Self{
        self.default = default.to_string();
        self.required = required;
        self
    }

    pub fn set_argtype(mut self, arg_type: ArgType) -> Self{
        self.arg_type = arg_type;
        self
    }

    pub fn add_alias(mut self, alias: &'a str) -> Self{
        self.alias.push(alias.to_owned());
        self
    }

}









#[derive(Clone, Debug)]
pub enum ArgValue {
    STR(String),
    VEC(Vec<String>)
}

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
            _ => { return Err(anyhow!("Argument is not scalar")) },
        }
    }

    pub fn get_bool(&self) -> Result<bool, anyhow::Error> {
        let v = self.match_type()?
            .parse::<bool>()
            .unwrap_or_else( 
                |err| panic!("Problem parsing arguments{}", err)
            );
        Ok(v)
    }

    pub fn get_f32(&self) -> Result<f32, anyhow::Error> {
        let v = self.match_type()?
            .parse::<f32>()
            .unwrap_or_else( 
                |err| panic!("Problem parsing arguments{}", err)
            );
        Ok(v)
    }

    pub fn get_i32(&self) -> Result<i32, anyhow::Error> {
        let v = self.match_type()?
            .parse::<i32>()
            .unwrap_or_else( 
                |err| panic!("Problem parsing arguments{}", err)
            );
        Ok(v)
    }

    pub fn get_string(&self) -> Result<String, anyhow::Error> {
        let v = self.match_type()?;
        Ok(v)
    }

    pub fn get_vec(&self) -> Result<Vec<String>, anyhow::Error> {
        match self {
            ArgValue::VEC(v) => Ok(v.clone()),
            _ => { return Err(anyhow!("Argument is not vector")) },
        }
    }

    pub fn push_vec(&mut self, item: String) -> Result<(), anyhow::Error> {
        match self {
            ArgValue::VEC(v) => v.push(item),
            _ => { return Err(anyhow!("Argument is not vector")) },
        };
        Ok(())
    }

}

