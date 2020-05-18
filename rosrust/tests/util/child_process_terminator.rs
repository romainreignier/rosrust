use std::process::{Child, Command, Stdio};

#[must_use]
pub struct ChildProcessTerminator(pub Child);

impl ChildProcessTerminator {
    pub fn spawn(command: &mut Command) -> ChildProcessTerminator {
        command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        ChildProcessTerminator(command.spawn().unwrap())
    }

    #[allow(dead_code)]
    pub fn spawn_example(command: &mut Command) -> ChildProcessTerminator {
        assert!(Command::new("cargo")
            .arg("build")
            .arg("--all-targets")
            .output()
            .unwrap()
            .status
            .success());

        Self::spawn(command)
    }

    #[allow(dead_code)]
    pub fn spawn_example_bench(command: &mut Command) -> ChildProcessTerminator {
        assert!(Command::new("cargo")
            .arg("build")
            .arg("--all-targets")
            .arg("--release")
            .output()
            .unwrap()
            .status
            .success());

        Self::spawn(command)
    }
}

impl Drop for ChildProcessTerminator {
    fn drop(&mut self) {
        self.0.kill().unwrap();
    }
}
