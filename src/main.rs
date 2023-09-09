use std::process;
fn main() {
    println!("Comment of the commit: ");
    let stdin = std::io::stdin();
    let mut comment = String::new();
    // process::Command::new("pwd").spawn().expect("Could not pwd");

    stdin.read_line(&mut comment).unwrap();
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
    process::Command::new("git")
        .args(["commit", "push"])
        .spawn()
        .expect("Could not push");
}
