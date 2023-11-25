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
    println!("1. 🚀 Initial commit");
    println!("2. 📁 Add files");
    println!("3. 🐛 Bug fix");
    println!("4. 🎉 Feature implementation");
    println!("5. 🛠️ Refactoring");
    println!("6. ⚡ Performance optimization");
    println!("7. 🔄 Dependency updates");
    println!("8. 📚 Documentation");
    println!("9. 🗑️ File deletion");
    println!("10. 🌿 Branch merging");
    println!("11. ⏪ Revert commit");
    println!("12. 🤖 Github action");
    println!("13. 🐋 Docker");
    println!("14. ☸️ Kubernetes");
    println!("15. 🏗️ Terraform");

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
        "12" => "🤖".to_string(),
        "13" => "🐋".to_string(),
        "14" => "☸️".to_string(),
        "15" => "🏗️".to_string(),
        _ => "😶".to_string(),
    }
}

fn commit_process(comment: String, emoji: String) {
    let capitalized = capitalize_first_letter(&comment);
    let processed_comment = [emoji, capitalized].join(" ");

    process::Command::new("git")
        .args(["commit", "-m", &processed_comment])
        .spawn()
        .expect("Could not commit");
}

fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

fn main() {
    let comment = read_comment();
    let emoji = set_emoji();
    commit_process(comment, emoji);
}
