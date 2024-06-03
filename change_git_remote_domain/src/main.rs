use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Change this to the desired directory path
    let home_dir = env::var("HOME").expect("$HOME environment variable not set");
    let dir_path = format!("{}/Workspace/Test", home_dir);
    let doc_dir = Path::new(dir_path.as_str());

    if doc_dir.is_dir() {
        for entry in doc_dir.read_dir().expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            let repo_path = entry.path();

            if repo_path.is_dir() {
                let repo_dir = repo_path.file_name().unwrap().to_str().unwrap().to_owned();
                let git_dir = repo_path.join(".git");

                if git_dir.exists() {
                    println!("Processing repository: {}", repo_dir);
                    change_remote_domain(&repo_path, &repo_dir);
                }
            }
        }
    } else {
        println!("The directory '{}' does not exist.", dir_path);
    }
}

fn change_remote_domain(repo_path: &Path, repo_dir: &str) {
    let remote_output = Command::new("git")
        .current_dir(repo_path)
        .arg("remote")
        .arg("-v")
        .output()
        .expect("Failed to execute git remote");

    if !remote_output.status.success() {
        println!("Error: Failed to get remote information for {}", repo_dir);
        return;
    }

    let remote_output_str = String::from_utf8_lossy(&remote_output.stdout);
    let remote_lines: Vec<&str> = remote_output_str.lines().collect();

    for line in remote_lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let remote_name = parts[0];
            let old_remote_url = parts[1];

            let new_remote_url = update_remote_domain(old_remote_url);

            if new_remote_url != old_remote_url {
                let set_remote_output = Command::new("git")
                    .current_dir(repo_path)
                    .arg("remote")
                    .arg("set-url")
                    .arg(remote_name)
                    .arg(&new_remote_url)
                    .output()
                    .expect("Failed to execute git remote set-url");

                if set_remote_output.status.success() {
                    println!(
                        "Remote URL for '{}' updated from '{}' to '{}'",
                        remote_name, old_remote_url, new_remote_url
                    );
                } else {
                    println!(
                        "Error: Failed to update remote URL for '{}' in repository {}",
                        remote_name, repo_dir
                    );
                }
            }
        }
    }
}

fn update_remote_domain(remote_url: &str) -> String {
    // Replace this with your logic to update the remote domain
    // For example, you can use regex to replace the domain part of the URL
    remote_url.replace("old.domain.com", "new.domain.com")
}