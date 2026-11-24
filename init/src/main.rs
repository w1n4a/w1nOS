use equ::{self, Core, Types};
use std::{
    collections::HashMap,
    env, fs,
    io::{self, Write},
    process::Command,
    thread::sleep,
    time::Duration,
};

use color_print::{cformat, cprint, cprintln};
use passterm::prompt_password_stdin;
use sha2::{Digest, Sha256};

use crate::Stream::{Authication, Pass};

enum Stream {
    Authication,
    Pass,
}

fn main() {
    let user_file = equ::new("/config/user.dat").load();
    let mut user_data: HashMap<String, Types> = HashMap::new();

    match user_file {
        Some(v) => {
            user_data = v;
        }
        None => {
            fs::File::create("config/user.dat").unwrap();
        }
    }

    let password: Option<&String> = match user_data.get("password") {
        Some(v) => match v {
            Types::String(v) => Some(v),
            Types::Vector(_) => None,
        },
        None => None,
    };

    match password {
        Some(p) => {
            thread(p, Authication);
        }
        _ => login(),
    }
}

fn thread(password: &str, stream: Stream) {
    env::set_current_dir("/user").unwrap();

    if let Authication = stream {
        loop {
            let input = getpass(cformat!("<b>Enter password:</> "));

            if hash(&input) == password.trim() {
                break;
            }
        }
    }
    Command::new("/programs/core/shell").status().unwrap();
    cprintln!("<r>KERNEL PANIC: </> please write us on github");
    loop {
        sleep(Duration::from_secs(1));
    }
}

fn login() {
    cprintln!("<m><bold>Login new user</></>\n");

    let mut data: HashMap<String, Types> = HashMap::new();

    let name = input(cformat!("<b>Name:</> "));

    if name.trim().len() == 0 {
        cprintln!("<r><bold>Invalid name</></>\n");
        login();
    } else {
        data.insert("name".to_string(), Types::String(name.trim().to_string()));
    }

    let mut pass = getpass(cformat!("<b>Password:</> "));
    let pass2 = getpass(cformat!("<b>Confirmation:</> "));

    if pass.trim().len() == 0 || pass != pass2 {
        cprintln!("<r><bold>Invalid password</></>\n");
        login();
    } else {
        pass = hash(&pass);
        data.insert("password".to_string(), Types::String(pass.clone()));
    }

    let file = equ::new("/config/user.dat");
    file.write(&data).unwrap();

    thread(&pass.trim(), Pass);
}

fn hash(arg: &str) -> String {
    format!("{:x}", Sha256::digest(arg.trim()))
}

fn input(text: String) -> String {
    let mut input: String = String::new();
    cprint!("{}", &text);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input
}

fn getpass(text: String) -> String {
    let pass =
        prompt_password_stdin(Some(&cformat!("{}", &text)), passterm::Stream::Stdout).unwrap();
    pass
}
