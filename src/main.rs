use std::io::{self, BufRead, BufReader};
use std::os::unix::process::CommandExt;

fn main() -> io::Result<()> {
    let mut args = std::env::args_os();
    args.next();
    let script = args.next();
    match script {
        Some(script) => interpret(script.as_ref(), args),
        None         => {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "No script file provided!"))
        }
    }
}

fn interpret<T>(script : &std::path::Path, args : T) -> io::Result<()>
    where
        T: Iterator<Item = std::ffi::OsString>
    {
    let f = std::fs::File::open(script)?;
    let f = BufReader::new(f);
    let mut prefix : Option<String> = None;
    let mut sh : String = String::new();

    for err_line in f.lines() {
        let line = err_line?;
        if line.starts_with("#!") {
            continue;
        }
        match &prefix {
            None => {
                match line.find(char::is_whitespace) {
                    None => continue,
                    Some(end) => {
                        let p = &line[..end];
                        prefix = Some(p.to_owned());
                        sh.push_str(line.strip_prefix(&p).unwrap());
                        sh.push('\n');
                    }
                }
            },
            Some(pre) => {
                match line.strip_prefix(pre) {
                    None => break,
                    Some(rest) => {
                        sh.push_str(rest);
                        sh.push('\n');
                    }
                }
            }
        }
    }
    Err(std::process::Command::new("/bin/sh").arg("-c").arg(sh).arg(script).args(args).exec())
}
