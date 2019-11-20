use std::process::Command;

fn main() -> Result<(), std::io::Error> {
    let mvn_test = Command::new("mvn").arg("test").output();
    let mvn_test = mvn_test.expect("command `mvn test` not found");
    println!("{}", String::from_utf8(mvn_test.stdout).unwrap());

    if mvn_test.status.success() {

        let git_add = Command::new("git").arg("add").arg("-A").output();
        let git_add = git_add.expect("command `git add -A` failed");
        println!("{}", String::from_utf8(git_add.stdout).unwrap());
        
        let git_commit = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("\"solve step\"")
        .output();

        let _ = git_commit.expect("command `git commit` failed");
        
        let git_merge = Command::new("git")
        .arg("merge")
        .arg("--no-ff")
        .arg("step2")
        .output();

        let git_merge = git_merge.expect("command `git merge` failed");
        println!("{}", String::from_utf8(git_merge.stdout).unwrap());
    } else {
        eprint!("Test failedtry again ! ");
    }

    Ok(())
}
