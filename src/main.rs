struct EchoOptions {
    add_newline_at_end: bool,
    interpret_escaped_characters: bool,
    show_help_info: bool,
    show_version_info: bool,
}

impl EchoOptions {
    fn new() -> EchoOptions {
        EchoOptions {
            add_newline_at_end: true,
            interpret_escaped_characters: false,
            show_help_info: false,
            show_version_info: false,
        }
    }
}

fn print_version_info() -> String {
    format!("Version {}", env!("CARGO_PKG_VERSION"))
}

fn print_help_info() -> String {
    format!(
        r#"
        DESCRIPTION:
        Print Strings passed as arguments to the standard output.

        USAGE:
        {} [OPTION] ... [STRING] ...

        VERSION:
        {}

        AUTHOR:
        {}

        OPTIONS:
         -n         Don't add newline at the end of the output (default is to add the line).
         -e         Process escaped sequences (default is to ignore).
         --version  Show installed version of the program.
         --help     Display this help message.

        ESCAPED SEQUENCES:
            \\      backslash
            \a      alert (BEL)
            \b      backspace
            \c      produce no further output
            \e      escape
            \f      form feed
            \n      new line
            \r      carriage return
            \t      horizontal tab
            \v      vertical tab
            \0NNN   byte with octal value NNN (1 to 3 digits)
            \xHH    byte with hexadecimal value HH (1 to 2 digits)
        "#,
        env!("CARGO_PKG_NAME"),
        print_version_info(),
        env!("CARGO_PKG_AUTHORS")
    )
}

fn process_args(v: &Vec<String>) -> (EchoOptions, Vec<&String>) {
    let mut args = v.into_iter();
    let mut s: Vec<&String> = vec![];
    let mut o = EchoOptions::new();

    while let Some(a) = args.next() {
        if a == "-n" {
            o.add_newline_at_end = false;
        } else if a == "-e" {
            o.interpret_escaped_characters = true;
        } else if a == "--version" || a == "-v" {
            o.show_version_info = true;
            break;
        } else if a == "--help" || a == "-h" {
            o.show_help_info = true;
            break;
        } else if a.starts_with("-") {
            println!("Ignoring unknown option: {a}");
        } else {
            s.push(a);
        }
    }
    (o, s)
}

fn echo(args: &Vec<String>) -> String {
    let mut s = String::new();
    let (opts, v) = process_args(&args);
    if opts.show_version_info {
        s.push_str(print_version_info().as_str());
    } else if opts.show_help_info {
        s.push_str(print_help_info().as_str());
    } else {
        let mut v_len = v.len();
        for arg in v {
            if !opts.interpret_escaped_characters {
                s.push_str(arg);
                continue;
            }
            let mut ct = arg.chars().into_iter();
            while let Some(c) = ct.next() {
                let a = if c == '\\' {
                    if let Some(e) = ct.next() {
                        match e {
                            '\\' => '\\',
                            'a' => '\u{7}',
                            'b' => '\u{8}',
                            'c' => '\u{04}',
                            'e' => '\u{1b}',
                            'f' => '\u{0c}',
                            'n' => '\u{0a}',
                            'r' => '\u{0d}',
                            't' => '\u{09}',
                            'v' => '\u{0b}',
                            _ => '\0',
                        }
                    } else {
                        '0'
                    }
                } else {
                    c
                };
                s.push(a);
            }
            v_len = v_len - 1;
            if v_len != 0 {
                print!(" ");
            }
        }
    }
    if opts.add_newline_at_end {
        s.push('\u{a}')
    }
    s
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    print!("{}", echo(&args.collect::<Vec<String>>()));
}

// #[cfg(test)]
// mod echo_test {
//     use super::*;

//     fn prints_version_info() {}
// }
