use crate::Executor;
use crate::Shell;

use std::{fs, io};

pub struct LsCommand {
    pub all: bool,
}

impl Executor for LsCommand {
    fn execute(&self, shell: &mut Shell) {
        let mut entries = fs::read_dir(shell.current_dir.to_str().unwrap())
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();

        entries.sort();

        for entry in entries {
            let path = entry
                .strip_prefix(shell.current_dir.to_str().unwrap())
                .unwrap();
            if !&self.all {
                if path.to_str().unwrap().starts_with(".") {
                    continue;
                }
            }
            println!("{}", path.to_str().unwrap());
        }
    }
}
