use crate::Executor;
use crate::Shell;

use std::path::Path;

pub struct CdCommand<'a> {
    pub arg: &'a str,
}

impl Executor for CdCommand<'_> {
    fn execute(&self, shell: &mut Shell) {
        let path = Path::new(&self.arg);
        if path.is_absolute() {
            shell.current_dir = path.canonicalize().unwrap();
        } else {
            let abspath = &shell.current_dir.as_path().join(path);
            match abspath.canonicalize() {
                Ok(td) => shell.current_dir = td,
                Err(_) => println!("No such file or directory"),
            }
        }
    }
}
