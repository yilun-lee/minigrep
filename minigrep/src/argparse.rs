use anyhow::anyhow;
use anyhow::Result;
use argtool::{ArgGroup, ArgItem, ArgType};

/// an arg struct to store argument for usage usage
#[derive(Debug)]
pub struct MiniGrepArg {
    pub ahead_size: i32,
    pub behind_size: i32,
    pub line_num_flag: bool,
    pub file_path_flag: bool,
    pub color_flag: bool,

    pub expr: Vec<String>,
    pub extract_expr: Vec<String>,
    pub replace_expr: Vec<String>,
    pub replacer: Vec<String>,
    pub replace_times: usize,
    pub ignorecase: bool,

    pub file_path: String,
    pub skip_hidden: bool,
    pub max_depth: usize,
}

impl MiniGrepArg {
    /// create a new MiniGrepArg from sys_args
    pub fn new(sys_args: impl Iterator<Item = String>) -> Result<MiniGrepArg> {
        let mut my_arg_table = get_parser()?;
        my_arg_table.parse(sys_args)?;

        // deal with positional and list expr
        let expr_pos = my_arg_table.get_string("expr_pos")?;
        let mut expr = my_arg_table.get_vec("expr")?;
        expr.push(expr_pos);

        let my_arg = MiniGrepArg {
            ahead_size: my_arg_table.get_i32("ahead")?,
            behind_size: my_arg_table.get_i32("behind")?,
            line_num_flag: my_arg_table.get_bool("line_num")?,
            file_path_flag: my_arg_table.get_bool("file_name")?,
            color_flag: my_arg_table.get_bool("color_flag")?,

            expr,
            extract_expr: my_arg_table.get_vec("extract_expr")?,
            replace_expr: my_arg_table.get_vec("replace_expr")?,
            replacer: my_arg_table.get_vec("replacer")?,
            replace_times: my_arg_table.get_i32("replace_times")? as usize,
            ignorecase: my_arg_table.get_bool("ignorecase")?,

            file_path: my_arg_table.get_string("file_path")?,
            skip_hidden: my_arg_table.get_bool("skip_hidden")?,
            max_depth: my_arg_table.get_i32("max_depth")? as usize,
        };

        if my_arg.replace_expr.len() != my_arg.replacer.len() {
            return Err(anyhow!(
                "replace_expr and replacer is not the same length, {} != {}",
                my_arg.replace_expr.len(),
                my_arg.replacer.len()
            ));
        }
        Ok(my_arg)
    }
}

/// get arg for minigrep
fn get_parser() -> Result<ArgGroup> {
    let name = String::from("Minigrep");
    let desciption = String::from("This is a minigrep implemented for fun.");

    let mut my_arg_table: ArgGroup = ArgGroup::new(name, desciption);

    my_arg_table.add_arg(
        ArgItem::new("ahead", "A")
            .set_detail("print N line ahead of match line ")
            .set_default("0", false)
            .set_argtype(ArgType::BaseType),
    )?;

    my_arg_table.add_arg(
        ArgItem::new("behind", "B")
            .set_detail("print N line behind match line ")
            .set_default("0", false)
            .set_argtype(ArgType::BaseType),
    )?;

    my_arg_table.add_arg(
        ArgItem::new("line_num", "n")
            .set_detail("print line number ahead ")
            .set_default("false", false)
            .set_argtype(ArgType::FlagType),
    )?;

    my_arg_table.add_arg(
        ArgItem::new("file_name", "f")
            .set_detail("print file name ahead ")
            .set_default("false", false)
            .set_argtype(ArgType::FlagType),
    )?;

    my_arg_table.add_arg(
        ArgItem::new("color_flag", "C")
            .set_detail("paint matched ")
            .set_default("true", false)
            .set_argtype(ArgType::FlagType),
    )?;

    my_arg_table.add_arg(
        ArgItem::new("expr_pos", "e")
            .set_detail("re expression, required ")
            .set_argtype(ArgType::PositionalType),
    )?;

    my_arg_table.add_arg(
        ArgItem::new("expr", "M")
            .set_detail("re expression ")
            .set_default("", false)
            .set_argtype(ArgType::ListType),
    )?;

    my_arg_table.add_arg(
        ArgItem::new("extract_expr", "E")
            .set_detail("re expression for extract pattern ")
            .set_default("", false)
            .set_argtype(ArgType::ListType),
    )?;

    my_arg_table.add_arg(
        ArgItem::new("replace_expr", "R")
            .set_detail("re expression for replcae pattern")
            .set_default("", false)
            .set_argtype(ArgType::ListType),
    )?;

    my_arg_table.add_arg(
        ArgItem::new("replacer", "r")
            .set_detail("String for replcae pattern with, should be the same len with replace_expr")
            .set_default("", false)
            .set_argtype(ArgType::ListType),
    )?;

    my_arg_table.add_arg(
        ArgItem::new("replace_times", "t")
            .set_detail("replace time for replace pattern ")
            .set_default("-1", false)
            .set_argtype(ArgType::BaseType),
    )?;

    my_arg_table.add_arg(
        ArgItem::new("ignorecase", "i")
            .set_detail("ignorecase or not ")
            .set_default("false", false)
            .set_argtype(ArgType::FlagType),
    )?;

    my_arg_table.add_arg(
        ArgItem::new("skip_hidden", "H")
            .set_detail("skip hidden files ")
            .set_default("true", false)
            .set_argtype(ArgType::FlagType),
    )?;

    my_arg_table.add_arg(
        ArgItem::new("file_path", "F")
            .set_detail("File to be matched, required ")
            .set_argtype(ArgType::PositionalType),
    )?;

    my_arg_table.add_arg(
        ArgItem::new("max_depth", "d")
            .set_detail("depth for os walker ")
            .set_default("10", false)
            .set_argtype(ArgType::BaseType),
    )?;

    Ok(my_arg_table)
}
