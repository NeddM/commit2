use std::process;
fn main() {
    println!("Comment of the commit: ");
    let stdin = std::io::stdin();
    let mut comment = String::new();

    match stdin.read_line(&mut comment) {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    }
    commit_process(comment);

    push_process();
}

fn commit_process(comment: String) {
    println!("{}", comment);

    let processed_comment = ["emoji", &comment].join(" ");

    process::Command::new("git")
        .args(["commit", "-m", &processed_comment])
        .spawn()
        .expect("Could not commit");
}

fn push_process() {
    let stdin = std::io::stdin();
    let mut wanna_push = String::new();
    println!("You want to push the commit? (y/n): ");
    match stdin.read_line(&mut wanna_push) {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    }

    if wanna_push.to_uppercase() == "Y" {
        process::Command::new("git")
            .arg("push")
            .spawn()
            .expect("Could not push");
    }
}
