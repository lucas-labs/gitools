use {
    eyre::Result,
    lool::fail,
    std::{
        env::{args, current_dir, current_exe},
        path::{Path, PathBuf},
    },
};

#[derive(Debug)]
pub struct ExecutionContext {
    pub home: PathBuf,
    pub cwd: PathBuf,
    pub args: Vec<String>,
}

impl ExecutionContext {
    /// Create new execution context
    pub fn new() -> Result<ExecutionContext> {
        let executable_path = current_exe()?;

        let home = executable_path.parent().unwrap_or(Path::new(".")).to_path_buf();
        let args: Vec<String> = args().skip(1).collect();

        // if we have a --cwd argument, use it as the current working directory
        // otherwise, use current_dir()
        let cwd = if let Some(cwd) = args.iter().position(|arg| arg == "--cwd") {
            let cwd = Path::new(&args[cwd + 1]);
            if !cwd.is_dir() {
                return fail!("--cwd argument is not a directory: {:?}", cwd);
            }
            cwd.to_path_buf()
        } else {
            current_dir()?
        };

        Ok(ExecutionContext { home, cwd, args })
    }
}
