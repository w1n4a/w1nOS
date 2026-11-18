#![allow(dead_code)]

use equ::{self, Core, Types};
use std::{collections::HashMap, io};

pub const PATH_PATH: &str = "/config/PATH.dat";

pub type Args = Vec<Arg>;

pub struct App {
    pub name: String,
    pub len: usize,
    pub main: fn(Args),
}

pub struct Arg {
    pub value: String,
    pub pos: usize,
}

pub trait Run {
    fn run(&self, args: &str);
    fn name(&self, args: &str) -> bool;
}

pub trait ArgsFn {
    fn arg(&self, id: usize) -> Option<&Arg>;
}

impl ArgsFn for Args {
    fn arg(&self, id: usize) -> Option<&Arg> {
        for i in 0..self.len() {
            if i == id - 1 {
                return Some(&self[i]);
            }
        }
        return None;
    }
}

impl Run for App {
    fn run(&self, args: &str) {
        let mut tmp: Vec<&str> = args.split_whitespace().collect();
        tmp.remove(0);

        if self.len != 0 && self.len == tmp.len() {
            let mut args: Vec<Arg> = Vec::new();

            for v in 0..tmp.len() {
                let arg = Arg {
                    value: tmp[v].to_string(),
                    pos: v,
                };

                args.push(arg);
            }

            (self.main)(args);
        };
    }
    fn name(&self, args: &str) -> bool {
        let tmp: Vec<&str> = args.trim().split_whitespace().collect();
        if tmp.len() > 0 {
            let i = tmp.get(0).unwrap();

            if i.trim() == self.name.trim() {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

pub struct PATH {
    pub path: String,
    pub data: HashMap<String, String>,
}

pub trait PathFn {
    fn add(&self, name: &str, path: &str) -> Result<(), io::Error>;
    fn del(&self, obj: &str) -> Result<(), io::Error>;
}

impl PATH {
    pub fn new() -> Self {
        Self {
            path: PATH_PATH.to_string(),
            data: {
                let mut map: HashMap<String, String> = HashMap::new();
                let data = equ::new(PATH_PATH).load().unwrap();
                for i in data {
                    match i.1 {
                        Types::String(v) => {
                            map.insert(i.0, v);
                        }
                        _ => {}
                    }
                }
                map
            },
        }
    }
}

impl PathFn for PATH {
    fn add(&self, name: &str, path: &str) -> Result<(), io::Error> {
        let data = equ::new(&self.path);
        let mut map: HashMap<String, Types> = HashMap::new();
        map.insert(name.to_string(), Types::String(path.to_string()));
        data.add(&map)
    }

    fn del(&self, obj: &str) -> Result<(), io::Error> {
        let mut tmp: HashMap<String, Types> = HashMap::new();
        let file = equ::new(&self.path);
        let data = file.load().unwrap();
        for i in data {
            if &i.0 == obj
                || match &i.1 {
                    Types::String(v) => v == obj,
                    Types::Vector(_) => false,
                }
            {
                continue;
            } else {
                tmp.insert(i.0, i.1);
            }
        }
        file.write(&tmp)
    }
}
