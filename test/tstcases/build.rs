use std::env;
use extutils::fileop::*;
use extutils::timeop::{get_time_local_str};

#[allow(unreachable_code)]
fn get_git_hash() -> Option<String> {
    use std::process::Command;

    let branch = Command::new("git")
                         .arg("rev-parse")
                         .arg("--abbrev-ref")
                         .arg("HEAD")
                         .output();
    if let Ok(branch_output) = branch {
        let branch_string = String::from_utf8_lossy(&branch_output.stdout);
        let commit = Command::new("git")
                             .arg("rev-parse")
                             .arg("--verify")
                             .arg("HEAD")
                             .output();
        if let Ok(commit_output) = commit {
            let commit_string = String::from_utf8_lossy(&commit_output.stdout);

            return Some(format!("{}, {}",
                        branch_string.lines().next().unwrap_or(""),
                        commit_string.lines().next().unwrap_or("")));
        } else {
            panic!("Can not get git commit:");
        }
    } else {
        panic!("Can not get git branch:");
    }
    None
}

fn main() {
    let mut outs :String = "".to_string();
    if let Some(git) = get_git_hash() {
        outs.push_str(&format!("const GIT_HASH :&str = \"{}\";\n",git));
    }
    if let Ok(tms) = get_time_local_str() {
        outs.push_str(&format!("const COMPILE_TIME :&str =\"{}\";\n",tms));    
    }
    
    //outs.push_str("const VERSION_INFO :&str =\"2.0.2\";\n");
    let _ = write_file("src/version.rs",&outs);
    env::set_var("COMPILE_TIME","2024-07-15");
}