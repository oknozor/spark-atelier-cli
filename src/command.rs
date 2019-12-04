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
            .arg(&format!("origin/step{}", step + 1))
            .output()
            .map_err(|e| e.into())
    }
}

pub mod maven {
    use crate::error::Error;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::process::Command;
    use std::process::Stdio;

    pub fn test() -> Result<bool, Error> {
        let mut child = Command::new("mvn")
            .arg("test")
            .stdout(Stdio::piped())
            .spawn()?;

        BufReader::new(child.stdout.take().unwrap())
            .lines()
            .for_each(|line| println!("{}", line.unwrap_or("".into())));

        let success = child.wait()?.success();
        Ok(success)
    }
}
