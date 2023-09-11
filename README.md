# commit2

This is a simple Rust script called "commit2" that helps you create and commit to Git repositories with a custom emoji and comment. This script prompts you to enter a comment for your commit and then allows you to select an emoji to accompany the comment. It then commits your changes with the chosen emoji and comment using the Git command-line tool.

## Prerequisites

Before using this script, ensure that you have the following prerequisites installed on your system:

-   Rust: You can install Rust by following the instructions on the official website: Rust Installation.
-   Git: Make sure Git is installed on your system and configured with the repository where you want to make commits.

## How to Use

Follow these steps to use the "commit2" script:

1. Clone or navigate to the Git repository where you want to make commits.

2. Save the "commit2" Rust script to a file, e.g., commit2.rs.

3. Compile the Rust script by running the following command:

```shell
cargo build --release
```

4. Save the exe on `System32` in Windows or `/usr/local/bin/` in Linux.

5. Run the compiled executable:

```shell
commit2
```

6. Follow the on-screen prompts:

-   You will be prompted to enter a comment for your commit.
-   You will then be presented with a list of emojis to choose from. Enter the corresponding number to select an emoji for your commit.
-   The script will create a Git commit using the selected emoji and comment.

## Emoji Options

The following emojis are available for your commit messages:

1. 🚀 initial commit
2. 📁 add files
3. 🐛 bug fix
4. 🎉 feature implementation
5. 🛠️ refactoring
6. ⚡ performance optimization
7. 🔄 dependency updates
8. 📚 documentation
9. 🗑️ file deletion
10. 🌿 branch merging
11. ⏪ revert commit

If you enter an invalid option, the script will use a default emoji (😶).
