use git2::IndexAddOption;
use git2::Repository;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), git2::Error> {

    let mvn_test  = Command::new("mvn").arg("test").output();
    let mvn_test  = mvn_test.expect("command `mvn test` not found");
    println!("{}", String::from_utf8(mvn_test.stdout).unwrap());

    // Get repository on the current path
    let repo = match Repository::open(&Path::new(".")) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    // This callback is called on each out of index file
    let git_add_callback = &mut |path: &Path, _matched_spec: &[u8]| -> i32 {
        let status = repo.status_file(path).expect("unable to get `git status`");

        if status.contains(git2::Status::WT_MODIFIED) || status.contains(git2::Status::WT_NEW) {
            0 // Flag new or modified files for add
        } else {
            1 // deny other
        }
    };

    // get the current repo index
    let mut index = repo.index()?;

    // get the current repo index
    if mvn_test.status.success() {
        println!("ok");
        index
            .add_all(vec!["."], IndexAddOption::DEFAULT, Some(git_add_callback))
            .unwrap();
    } else {
        eprintln!("No changes in the current index")
    }

    // flush changes
    index.write()?;
    Ok(())
}
