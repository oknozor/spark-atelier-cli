
use std::process::Command;

fn main() {
    let output = Command::new("mvn")
    .arg("test")
    .output()
    .expect("failed to execute process");

    println!("{}", output.status);
}