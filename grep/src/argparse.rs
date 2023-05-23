
use anyhow;
use argtool::{ArgGroup, ArgItem, ArgType};



/// an arg struct to store argument for usage usage
#[derive(Debug)]
pub struct MiniGrepArg {
    pub ahead_size: i32,
    pub behind_size: i32,
    pub expression: String,
    pub file_path: String,
    pub ignorecase: bool,
}


impl MiniGrepArg {
    /// create a new MiniGrepArg from sys_args
    pub fn new (sys_args: impl Iterator<Item = String>) -> anyhow::Result<MiniGrepArg>{
        let mut my_arg_table = get_parser();
        my_arg_table.parse(sys_args)?;
        Ok(MiniGrepArg{
            ahead_size: my_arg_table.get_i32("ahead")?,
            behind_size: my_arg_table.get_i32("behind")?,
            expression: my_arg_table.get_string("expression")?,
            file_path: my_arg_table.get_string("file_path")?,
            ignorecase: my_arg_table.get_bool("ignorecase")?,
        })
    }
}



/// get arg for minigrep
fn get_parser() -> ArgGroup {
    let name = String::from("Minigrep");
    let desciption = String::from("This is a minigrep implemented for fun.");
    
    let mut my_arg_table: ArgGroup = ArgGroup::new(name, desciption);
    
    my_arg_table.add_arg(
        ArgItem::new("ignorecase", "i")
        .set_detail("ignorecase or not ")
        .set_default("false", false)
        .set_argtype(ArgType::FlagType)
    ).unwrap();

    my_arg_table.add_arg(
        ArgItem::new("expression", "e")
        .set_detail("re expression, required ")
        .set_argtype(ArgType::PositionalType)
    ).unwrap();

    my_arg_table.add_arg(
        ArgItem::new("file_path", "f")
        .set_detail("File to be matched, required ")
        .set_argtype(ArgType::PositionalType)
    ).unwrap();

    my_arg_table.add_arg(
        ArgItem::new("ahead", "A")
        .set_detail("print N line ahead of match line ")
        .set_default("0", false)
        .set_argtype(ArgType::BaseType)
    ).unwrap();

    my_arg_table.add_arg(
        ArgItem::new("behind", "B")
        .set_detail("print N line behind match line ")
        .set_default("0", false)
        .set_argtype(ArgType::BaseType)
    ).unwrap();


    return my_arg_table; 
}

