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

    pub fn test(step: i32) -> Result<bool, Error> {
        let mut child = Command::new("mvn")
            .arg("test")
            .arg(step_to_letters(step))
            .stdout(Stdio::piped())
            .spawn()?;

        BufReader::new(child.stdout.take().unwrap())
            .lines()
            .for_each(|line| println!("{}", line.unwrap_or("".into())));

        let success = child.wait()?.success();
        Ok(success)
    }

    fn step_to_letters(step: i32) -> String {
        let spelled_step = match step {
            1 => Some("One"),
            2 => Some("Two"),
            3 => Some("Three"),
            4 => Some("Four"),
            5 => Some("Five"),
            6 => Some("Six"),
            7 => Some("Seven"),
            _ => None
        };

        if let Some(step) = spelled_step {
            format!("-Dtest=Step{}Test", step)
        } else {
            String::from("")
        }
    }
}
