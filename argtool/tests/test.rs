

#[cfg(test)]
mod tests {
    use argtool::argitem::{ArgItem,ArgType};
    use argtool::argroup::ArgGroup;

    fn get_my_argroup() -> ArgGroup {
        let name = String::from("Test");
        let desciption = String::from("This is a Test");
        
        let mut my_arg_table: ArgGroup = ArgGroup::new(name, desciption);
        
        my_arg_table.add_arg(
            ArgItem::new("line", "n")
            .set_detail("number of line")
            .set_default("-1", false)
            .set_argtype(ArgType::BaseType)
        ).unwrap();
        my_arg_table.add_arg(
            ArgItem::new("ignorecase", "i")
            .set_detail("ignorecase or not ")
            .set_default("false", false)
            .set_argtype(ArgType::FlagType)
        ).unwrap();
        my_arg_table.add_arg(
            ArgItem::new("expression", "e")
            .set_detail("re expression, required ")
            .set_argtype(ArgType::ListType)
        ).unwrap();
        my_arg_table.add_arg(
            ArgItem::new("file", "f")
            .set_detail("File to be matched, required ")
            .set_argtype(ArgType::PositionalType)
        ).unwrap();
        return my_arg_table; 
    }

    #[test]
    fn test_sucess() {
        let mut my_arg_table = get_my_argroup();

        let mystr: String = String::from("/lib.rs -i this_is_file -e \"^[A-Z]+\" --e \";$\" -n 3 ");
        let my_cmd_iter = mystr.split(" ").map(|a| a.to_owned()).into_iter();

        let my_arg: std::collections::HashMap<String, argtool::ArgValue> = my_arg_table.parse(my_cmd_iter).unwrap();
        println!("{:#?}", my_arg);
        println!("{}", my_arg_table);

        assert_eq!("this_is_file".to_string(), my_arg["file"].get_string().unwrap());
        assert_eq!(vec!["\"^[A-Z]+\"".to_string(), "\";$\"".to_string()], my_arg["expression"].get_vec().unwrap());
        assert_eq!(true, my_arg["ignorecase"].get_bool().unwrap());
        assert_eq!(3, my_arg_table.get_i32("line").unwrap());

    }

    #[test]
    #[should_panic(expected="Argument --t not implemented")]
    fn test_undefined_arg() {
        let mut my_arg_table = get_my_argroup();

        let mystr: String = String::from("/lib.rs -i this_is_file -e \"^[A-Z]+\" -e \";$\" -n 3 -t UNDEFINE");
        let my_cmd_iter = mystr.split(" ").map(|a| a.to_owned()).into_iter();

        let my_arg = my_arg_table.parse(my_cmd_iter).unwrap();
        println!("{:#?}", my_arg);

    }


    #[test]
    #[should_panic(expected="Not enough positional argument, expect 1, got 0")]
    fn test_no_required() {
        let mut my_arg_table = get_my_argroup();

        let mystr: String = String::from("/lib.rs -i -e \"^[A-Z]+\" -e \";$\" -n 3");
        let my_cmd_iter = mystr.split(" ").map(|a| a.to_owned()).into_iter();

        let my_arg = my_arg_table.parse(my_cmd_iter).unwrap();
        println!("{:#?}", my_arg);

    }


    #[test]
    fn test_default() {
        let mut my_arg_table = get_my_argroup();

        let mystr: String = String::from("/lib.rs -e \"^[A-Z]+\" -e \";$\" this_is_file");
        let my_cmd_iter = mystr.split(" ").map(|a| a.to_owned()).into_iter();

        let my_arg = my_arg_table.parse(my_cmd_iter).unwrap();
        println!("{:#?}", my_arg);

        assert_eq!(-1, my_arg["line"].get_i32().unwrap());
        assert_eq!(false, my_arg["ignorecase"].get_bool().unwrap());

    }


    #[test]
    #[should_panic(expected="No content found for --expression")]
    fn test_no_content() {
        let mut my_arg_table = get_my_argroup();

        let mystr: String = String::from("/lib.rs -e -i this_is_file ");
        let my_cmd_iter = mystr.split(" ").map(|a| a.to_owned()).into_iter();

        let my_arg = my_arg_table.parse(my_cmd_iter).unwrap();
        println!("{:#?}", my_arg);

        assert_eq!(-1, my_arg["line"].get_i32().unwrap());
        assert_eq!(false, my_arg["ignorecase"].get_bool().unwrap());

    }


}
