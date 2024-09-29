use std::fs;

fn main() {
    // copy the README.md file to the ../../target/{profile}/tldr/ directory and call it "remotes.md"
    let profile = std::env::var("PROFILE").unwrap();

    fs::create_dir_all(format!("../../target/{}/tldr", profile)).unwrap();
    fs::copy("README.md", format!("../../target/{}/tldr/br.md", profile)).unwrap();
}
