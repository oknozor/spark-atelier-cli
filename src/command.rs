pub mod git {
    use crate::error::Error;
    use std::process::Command;
    use std::process::Output;

    pub fn add() -> Result<Output, Error> {
        Command::new("git")
            .arg("add")
            .arg("-A")
            .output()
            .map_err(|e| e.into())
    }

    pub fn commit() -> Result<Output, Error> {
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("\"solve step\"")
            .output()
            .map_err(|e| e.into())
    }

    pub fn merge(step: i32) -> Result<Output, Error> {
        Command::new("git")
            .arg("merge")
            .arg("--no-ff")
            .arg(&format!("step{}", step + 1))
            .output()
            .map_err(|e| e.into())
    }
}

pub mod maven {
    use crate::error::Error;
    use std::process::Command;

    pub fn test() -> Result<(bool, String), Error> {
        let mvn_test = Command::new("mvn").arg("test").output()?;
        let success = mvn_test.status.success();
        let stdout = mvn_test.stdout;
        Ok((success, String::from_utf8(stdout).unwrap()))
    }
}
