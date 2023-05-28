
use std::{vec, fmt};
use std::collections::HashMap;
use crate::argitem::{ArgItem,ArgType,ArgValue};

use anyhow::anyhow;

pub struct ArgGroup {
    pub name: String,
    pub discription: String,

    pos_arg: Vec<ArgItem>,
    base_arg: HashMap<String, ArgItem>,
    alias_map: HashMap<String, String>,

    pub arg_map: HashMap<String, ArgValue>,
}

impl Default for ArgGroup {
    fn default() -> ArgGroup {
        ArgGroup {
            name: "".to_string(),
            discription: "".to_string(),
        
            pos_arg: vec![],
            base_arg: HashMap::new(),
            alias_map: HashMap::new(),

            arg_map: HashMap::new(),
        }
    }
}

// public function
impl ArgGroup  {

    pub fn new(name: String, discription: String) -> ArgGroup {
        ArgGroup {
            name: name, 
            discription: discription, 
            ..Default::default()
        }
    }

    pub fn add_arg(&mut self, mut argitem: ArgItem) -> Result<(),anyhow::Error>{
        // check first type
        match argitem.arg_type {
            ArgType::PositionalType => {
                // check required
                if ! argitem.required { 
                    return Err(anyhow!("Positional argument should be set as reauired")) };
                // add to pos_arg
                self.pos_arg.push(argitem);
            },
            // flag type
            ArgType::FlagType => {
                argitem.required = false;
                if argitem.default != String::from("true") {
                    argitem.default = "false".to_string();
                } 
                self.add_non_pos_arg(argitem);
            },
            // other
            _ => {
                self.add_non_pos_arg(argitem);
            }
        };
        Ok(())

    }

    pub fn parse(&mut self, mut sys_args: impl Iterator<Item = String>,  
        ) -> Result<HashMap<String,ArgValue>,anyhow::Error>{
        
        // remove first argument, should be the binary or script path
        if let None = sys_args.next() {
            return Err(anyhow!("No argument found")) }

        let mut cc = 0;
        let pos_num =self.pos_arg.len();

        loop {
            match sys_args.next() {
                None => break,
                Some(arg) => {
                    // continue if empty string
                    if &arg.len() == &0 {
                        continue;

                    // For pos type argument if available -> not start with - , not coupled argument
                    } else if &arg[..1] != "-" {
                        if cc >= pos_num {
                            return Err(anyhow!("Too many positional argument, expect {}", pos_num)) }
                        let argitem = &self.pos_arg[cc];
                        self.arg_map.insert(argitem.name.to_string(), ArgValue::STR(arg.to_string()));
                        cc += 1;

                    // start with "-" -> argument
                    } else if &arg[..1] == "-" {
                        // get keyname by remove --
                        let mut keyname: String = arg.trim_matches('-').to_string();
                        // alias to real name
                        keyname = self.alias_to_name(keyname);
                        // match
                        self.match_arg(keyname, &mut sys_args)?;
                    } 
                }
            }
        }
        // check pos arg number 
        if cc != pos_num {
            return Err(anyhow!("Not enough positional argument, expect {}, got {}", pos_num, cc)) }
        // set default and check required
        self.set_default()?;
        Ok(self.arg_map.clone())

    }


}


// private funcion
impl ArgGroup  {

    fn add_non_pos_arg(&mut self, argitem: ArgItem) {
        for i in &argitem.alias {
            self.alias_map.insert(i.to_owned(), argitem.name.to_owned());
        }
        self.base_arg.insert(argitem.name.to_owned(), argitem);
    }

    fn alias_to_name(&self, keyname: String) -> String {
        let mut new_key_name = &keyname;
        
        if self.alias_map.contains_key(&keyname) {
            new_key_name = &self.alias_map[&keyname];
        }
        return new_key_name.to_owned();
    }

    fn match_arg(&mut self, keyname: String, 
        sys_args: &mut impl Iterator<Item = String>) -> Result<(),anyhow::Error>{
        
        
        let arg_item = match self.base_arg.get(&keyname) {
            Some(v) => v,
            None => return Err(anyhow!("Argument --{} not implemented", keyname))
        };
        
        match arg_item.arg_type {
            ArgType::BaseType => {
                // get next 
                let arg_val: String = self.get_arg_content(sys_args, keyname.clone())?;
                self.arg_map.insert(keyname.to_string(), ArgValue::STR(arg_val.to_string()));
            },

            ArgType::ListType => {
                // get next 
                let arg_val: String  = self.get_arg_content(sys_args, keyname.clone())?;
                // insert to vec
                let v: &mut ArgValue = self.arg_map.entry(keyname.to_string())
                    .or_insert(ArgValue::VEC(vec![]));
                v.push_vec(arg_val.to_string())?;
            },

            ArgType::FlagType => {
                // revese flag if arg is presented
                let flag: String;
                if arg_item.default == "true" {
                    flag = "false".to_string()
                } else if arg_item.default == "false" {
                    flag = "true".to_string()
                } else { 
                    return Err(anyhow!("Flag argument --{} default value should be true / false, got {}", &keyname, &arg_item.default));}
                self.arg_map.insert(keyname.to_string(), ArgValue::STR(flag));
            },

            ArgType::PositionalType => return Err(anyhow!("PositionalType should not be in keyname match")),
        };
        Ok(())

    }

    fn get_arg_content(&self, sys_args: &mut impl Iterator<Item = String>, 
        keyname: String,) -> Result<String,anyhow::Error> {
        let arg = sys_args.next()
            .ok_or_else(|| anyhow!("No argument found for --{}", keyname))?;
        if &arg[..1] == "-"{
            return Err(anyhow!("No content found for --{}", keyname))};
        Ok(arg)
    }

    fn set_default(&mut self, ) -> Result<(),anyhow::Error>{
        for (keyname, argitem) in self.base_arg.iter() {
            match self.arg_map.get(keyname) {
                Some(_) => (),
                None => {
                    // check 
                    if argitem.required {
                        return Err(anyhow!("Argument --{} is required but not set",keyname))}; 
                    // for List Type
                    if let ArgType::ListType = argitem.arg_type  {
                        self.arg_map.insert(argitem.name.to_owned(),
                            ArgValue::VEC(vec![]));
                    } else {
                        self.arg_map.insert(argitem.name.to_owned(),
                            ArgValue::STR(argitem.default.to_string()));
                    };
                },
            };
        };
        Ok(())
    }
}


#[doc(hidden)]
impl fmt::Display for ArgGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "{}", self.discription)?;
        writeln!(f, "")?;

        writeln!(f, "Positional arg:")?;
        for argitme in &self.pos_arg{
            writeln!(f, "{}", argitme)?;
        }

        writeln!(f, "Other argument:")?;
        for (_, argitme) in &self.base_arg{
            writeln!(f, "{}", argitme)?;
        }

        writeln!(f, "Currunt  argument:")?;
        writeln!(f, "{:#?}", &self.arg_map)?;

        Ok(())
    }
}









/// for get value
impl <'a> ArgGroup  {

    pub fn get_bool(&self, keyname: &'a str) -> Result<bool, anyhow::Error> {
        let value: &ArgValue = self.arg_map.get(keyname)
            .ok_or_else(|| anyhow!("No value found for argument --{}", keyname))?;
        let output = value.get_bool()?;
        Ok(output)
    }

    pub fn get_f32(&self, keyname: &'a str) -> Result<f32, anyhow::Error> {
        let value: &ArgValue = self.arg_map.get(keyname)
            .ok_or_else(|| anyhow!("No value found for argument --{}", keyname))?;
        let output = value.get_f32()?;
        Ok(output)
    }

    pub fn get_i32(&self, keyname: &'a str) -> Result<i32, anyhow::Error> {
        let value: &ArgValue = self.arg_map.get(keyname)
            .ok_or_else(|| anyhow!("No value found for argument --{}", keyname))?;
        let output = value.get_i32()?;
        Ok(output)
    }

    pub fn get_string(&self, keyname: &'a str) -> Result<String, anyhow::Error> {
        let value: &ArgValue = self.arg_map.get(keyname)
            .ok_or_else(|| anyhow!("No value found for argument --{}", keyname))?;
        let output = value.get_string()?;
        Ok(output)
    }

    pub fn get_vec(&self, keyname: &'a str) -> Result<Vec<String>, anyhow::Error> {
        let value: &ArgValue = self.arg_map.get(keyname)
            .ok_or_else(|| anyhow!("No value found for argument --{}", keyname))?;
        let output = value.get_vec()?;
        Ok(output)
    }

}