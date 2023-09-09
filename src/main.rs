use std::process;
fn main() {
    println!("Comment of the commit: ");
    let stdin = std::io::stdin();
    let mut comment = String::new();

    stdin.read_line(&mut comment).unwrap();
    // println!("{}", comment);
    commit_process(comment)
}

fn commit_process(comment: String) {
    println!("{}", comment);
    process::Command::new("git commit -m ").args(["emoji", &comment]);
}
