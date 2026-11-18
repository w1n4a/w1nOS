use crate::sbew::{App, Args, ArgsFn, PATH, Run};
use std::{env, process::Command};

pub fn main(input: &str, name: &str, path: &PATH) {
    let cd = App {
        name: "cd".to_string(),
        len: 1,
        main: cd,
    };

    match input.trim() {
        "" => {}
        "me" => println!("{}", &name),
        "pwd" => println!("{:?}", env::current_dir().unwrap()),
        x if cd.name(x) => cd.run(x),
        _ => {
            let mut args: Vec<&str> = input.split_whitespace().collect();
            let name = args.remove(0);
            for i in &path.data {
                if name == i.0.trim() {
                    Command::new(i.1.trim()).args(&args).status().unwrap();
                }
            }
        }
    }
}

fn cd(args: Args) {
    let i = args.arg(1).unwrap();

    match env::set_current_dir(&i.value) {
        Err(_) => {
            if env::current_dir()
                .unwrap()
                .ends_with(format!("/{}", i.value))
            {
            } else {
                println!("This is not a dir")
            }
        }
        _ => {}
    }
}
