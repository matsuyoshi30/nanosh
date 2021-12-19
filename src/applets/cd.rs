use crate::Executor;
use crate::Shell;

use std::path::Path;

pub struct CdCommand<'a> {
    pub arg: &'a str,
}

impl Executor for CdCommand<'_> {
    fn execute(&self, shell: &mut Shell) {
        let path = Path::new(&self.arg);
        if path.is_relative() {
            let abspath = &shell.current_dir.as_path().join(path);
            shell.current_dir = abspath.canonicalize().unwrap();
        } else {
            shell.current_dir = path.canonicalize().unwrap();
        }
    }
}
