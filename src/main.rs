use std::process;

fn read_comment() -> String {
    println!("Welcome to commit2!");
    println!("Comment of the commit: ");
    let stdin = std::io::stdin();
    let mut comment = String::new();

    match stdin.read_line(&mut comment) {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    }
    comment
}

fn set_emoji() -> String {
    let stdin = std::io::stdin();
    let mut option = String::new();

    println!("Select an emoji to your comment");
    println!("1. 🚀 initial commit");
    println!("2. 📁 add files");
    println!("3. 🐛 bug fix");
    println!("4. 🎉 feature implementation");
    println!("5. 🛠️ refactoring");
    println!("6. ⚡ performance optimization");
    println!("7. 🔄 dependency updates");
    println!("8. 📚 documentation");
    println!("9. 🗑️ file deletion");
    println!("10. 🌿 branch merging");
    println!("11. ⏪ revert commit");

    println!("Select option");

    match stdin.read_line(&mut option) {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    }

    option = option.trim().to_string();

    match &option[..] {
        "1" => "🚀".to_string(),
        "2" => "📁".to_string(),
        "3" => "🐛".to_string(),
        "4" => "🎉".to_string(),
        "5" => "🛠️".to_string(),
        "6" => "⚡".to_string(),
        "7" => "🔄".to_string(),
        "8" => "📚".to_string(),
        "9" => "🗑".to_string(),
        "10" => "🌿".to_string(),
        "11" => "⏪".to_string(),
        _ => "😶".to_string(),
    }
}

fn commit_process(comment: String, emoji: String) {
    let processed_comment = [emoji, comment].join(" ");

    process::Command::new("git")
        .args(["commit", "-m", &processed_comment])
        .spawn()
        .expect("Could not commit");
}

fn main() {
    let comment = read_comment();
    let emoji = set_emoji();
    commit_process(comment, emoji);
}
