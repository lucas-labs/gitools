use {
    glob::glob,
    std::{
        env,
        fs::{self, OpenOptions},
        io::Write,
        path::{absolute, Path, PathBuf},
    },
    toml::Value,
};

const DEBUG: bool = false;

/// log any message to the log file
fn log_message(message: &str) {
    if !DEBUG {
        return;
    }

    let log_file_path = "build.log";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .expect("Failed to open log file");

    writeln!(file, "{}", message.replace("\\\\", "/")).expect("Failed to write to log file");
}

/// log an info message to the log file
fn inf(message: &str) {
    log_message(&format!("[info] {}", message));
}

/// log an error message to the log file
fn err(message: &str) {
    log_message(&format!("[error] {}", message));
}

fn main() {
    inf("Starting build.rs execution...");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.");

    // remove the log file if it exists
    let log_file_path = "build.log";
    if Path::new(log_file_path).exists() {
        fs::remove_file(log_file_path).expect("Failed to remove log file");
    }

    // Get the current workspace directory
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let workspace_toml =
        absolute(Path::new(&cargo_manifest_dir).join("..").join("..").join("Cargo.toml")).unwrap();
    let workspace_dir = workspace_toml.parent().unwrap();

    inf(&format!("--» Workspace Cargo.toml: {:?}", workspace_toml));
    inf(&format!("--» Workspace directory: {:?}", workspace_dir));

    // Read the workspace's Cargo.toml
    let workspace_content = match fs::read_to_string(&workspace_toml) {
        Ok(content) => content,
        Err(e) => {
            err(&format!("Failed to read workspace Cargo.toml: {:?}", e));
            panic!("Failed to read workspace Cargo.toml");
        }
    };

    let workspace_toml_value: Value = match workspace_content.parse() {
        Ok(value) => value,
        Err(e) => {
            inf(&format!("Invalid TOML in workspace Cargo.toml: {:?}", e));
            panic!("Invalid TOML in workspace Cargo.toml");
        }
    };

    // Extract member packages (including wildcard expansion)
    let members = workspace_toml_value["workspace"]["members"]
        .as_array()
        .expect("Failed to parse workspace members")
        .iter()
        .flat_map(|member| {
            let member_str = member.as_str().unwrap();
            if member_str.contains('*') {
                // Handle wildcard paths (e.g., cmds/*)
                inf(&format!("Expanding wildcard for: {}", member_str));
                
                glob(&format!("{}/{}", workspace_dir.display(), member_str))
                    .unwrap()
                    .filter_map(Result::ok)
                    .collect::<Vec<_>>()
            } else {
                vec![workspace_dir.join(member_str)]
            }
        })
        .collect::<Vec<PathBuf>>();

    inf(&format!("Members found: {:?}", members));

    // Prepare a vector to store binaries and their paths
    let mut binaries_info = Vec::new();

    // Iterate over each member package
    for member in members {
        let package_toml_path = member.join("Cargo.toml");

        // Log the current package being processed
        inf(&format!("Processing package: {:?}", package_toml_path));

        // Read each package's Cargo.toml
        if let Ok(package_content) = fs::read_to_string(&package_toml_path) {
            let package_toml_value: Value = match package_content.parse() {
                Ok(value) => value,
                Err(e) => {
                    err(&format!("Invalid TOML in package Cargo.toml: {:?}", e));
                    continue;
                }
            };

            // Log the content of the package's Cargo.toml
            inf(&format!("Package TOML content: {:?}", package_toml_value));

            // Check if it has a binary target
            if let Some(bin_targets) = package_toml_value.get("bin") {
                for bin in bin_targets.as_array().unwrap() {
                    let bin_name = bin.get("name").unwrap().as_str().unwrap();
                    binaries_info
                        .push((bin_name.to_string(), package_toml_path.display().to_string()));
                }
            } else {
                // By default, the bin name is the package name for non-library packages
                if let Some(package_name) =
                    package_toml_value.get("package").and_then(|pkg| pkg.get("name"))
                {
                    let bin_name = package_name.as_str().unwrap();
                    binaries_info
                        .push((bin_name.to_string(), package_toml_path.display().to_string()));
                } else {
                    err(&format!("No binary target found for package: {:?}", package_toml_path));
                }
            }
        } else {
            err(&format!("Failed to read: {:?}", package_toml_path));
        }
    }

    // Generate Rust code for the struct containing the binary information
    let out_dir = env::var("OUT_DIR").unwrap();
    inf(&format!("OUT_DIR: {:?}", out_dir));

    let dest_path = Path::new(&out_dir).join("binaries_info.rs");

    let mut generated_code = String::new();
    generated_code.push_str("#[derive(Clone)]\n");
    generated_code.push_str("pub struct BinaryInfo {\n");
    generated_code.push_str("    pub name: &'static str,\n");
    generated_code.push_str("    pub cargo_toml_path: &'static str,\n");
    generated_code.push_str("}\n\n");

    generated_code.push_str("pub static BINARIES: &[BinaryInfo] = &[\n");

    // sort the binaries by name alphabetically
    binaries_info.sort_by(|a, b| a.0.cmp(&b.0));

    for (bin_name, bin_path) in binaries_info {
        generated_code.push_str(&format!(
            "    BinaryInfo {{ name: \"{}\", cargo_toml_path: \"{}\" }},\n",
            bin_name,
            bin_path.replace('\\', "/")
        ));
    }
    generated_code.push_str("];\n");

    // Write the generated code to the output file
    fs::write(dest_path.clone(), &generated_code).expect("Failed to write binaries_info.rs");

    inf(&format!("Generated file: {:?}", dest_path));
    log_message(&format!("\n{}", generated_code));
}
