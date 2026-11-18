use equ::{self, Core, Types};
use std::{
    collections::HashMap,
    env, fs,
    io::{self, Write},
};

use color_print::{cformat, cprint, cprintln};
use passterm::prompt_password_stdin;
use sha2::{Digest, Sha256};

mod core;
mod sbew;

use crate::Stream::{Authication, Pass};
use sbew::PATH;

enum Stream {
    Authication,
    Pass,
}

fn main() {
    let user_file = equ::new("config/user.dat").load();
    let mut user_data: HashMap<String, Types> = HashMap::new();

    match user_file {
        Some(v) => {
            user_data = v;
        }
        None => {
            fs::File::create("config/user.dat").unwrap();
        }
    }

    let name: Option<&String> = match user_data.get("name") {
        Some(v) => match v {
            Types::String(v) => Some(v),
            Types::Vector(_) => None,
        },
        None => None,
    };

    let password: Option<&String> = match user_data.get("password") {
        Some(v) => match v {
            Types::String(v) => Some(v),
            Types::Vector(_) => None,
        },
        None => None,
    };

    match (name, password) {
        (Some(n), Some(p)) => {
            thread(n, p, Authication);
        }
        _ => login(),
    }
}

fn thread(name: &str, password: &str, stream: Stream) {
    env::set_current_dir("/user").unwrap();

    let path: PATH = PATH::new();

    if let Authication = stream {
        loop {
            let input = getpass(cformat!("<b>Enter password:</> "));

            if hash(&input) == password.trim() {
                cprintln!("Welcome to <m>w1nOS</>, <b>{}</>!", &name);
                break;
            }
        }
    }

    loop {
        let cur_temp = env::current_dir().unwrap();
        let cur: Vec<&str> = cur_temp.to_str().unwrap().split("/").collect();
        let text = cformat!("<m>{}@{}</> ) ", name, cur[cur.len() - 1]);

        let input = input(text);
        core::main(&input, name, &path);
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

    let file = equ::new("config/user.dat");
    file.write(&data).unwrap();

    thread(&name.trim(), &pass.trim(), Pass);
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
