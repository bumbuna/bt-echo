# ECHO

## DESCRIPTION
A Rust implementation of the comand-line utility echo.

## USAGE

```bash
$ echo [OPTIONS] [STRINGS]

$ echo Hello world
> Hello world

$ echo "Hello\nworld"
> Hello\nworld

$ echo -e "Hello world \n"
> Hello
> world
```

## FLAGS
-n         Don't add newline at the end of the output (default is to add the line).
-e         Process escaped sequences (default is to ignore).
--version  Show installed version of the program.
--help     Display this help message.

## ESCAPED SEQUENCES
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
\0NNN   byte with octal value NNN (1 to 3 digits) (TBD)
\xHH    byte with hexadecimal value HH (1 to 2 digits) (TBD)

## CONTRIBUTION
All contributions towards this project are welcomed through pull requests.

## ACKNOWLEDGEMENTS
https://zerotomastery.io/blog/rust-practice-projects/
