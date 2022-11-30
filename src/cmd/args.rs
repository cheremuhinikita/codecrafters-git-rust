pub fn get_named_arg<'a>(args: &[String], name: &'static str) -> (Vec<String>, Option<String>) {
    let args = args.to_vec();

    let name_idx = match args.iter().position(|a| a == name) {
        Some(position) => position,
        None => return (args, None),
    };

    let arg = match args.get(name_idx + 1) {
        Some(arg) => arg,
        None => return (args, None),
    };

    let args = [&args[..name_idx], &args[name_idx + 2..args.len()]].concat();
    let value = Some(arg.to_owned());

    (args, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_named_arg() {
        assert_eq!((vec![], None), get_named_arg(&vec![], "-p"));
        assert_eq!(
            (vec![String::from("-p")], None),
            get_named_arg(&vec![String::from("-p")], "-p")
        );
        assert_eq!(
            (vec![], Some(String::from("1212"))),
            get_named_arg(&vec![String::from("-p"), String::from("1212")], "-p")
        );
        assert_eq!(
            (
                vec![String::from("-w"), String::from("-s")],
                Some(String::from("1212"))
            ),
            get_named_arg(
                &vec![
                    String::from("-w"),
                    String::from("-p"),
                    String::from("1212"),
                    String::from("-s")
                ],
                "-p"
            )
        );
    }
}
