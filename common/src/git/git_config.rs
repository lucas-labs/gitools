use {
    eyre::{OptionExt, Result},
    std::{
        collections::HashMap,
        path::{Path, PathBuf},
    },
};

fn sanitize(s: String) -> String {
    s.trim_matches(|c: char| c == '[' || c == ']' || c == '"' || c.is_whitespace())
        .replace(['\t', '\n', '\r'], "")
}

/// get the section name and optionally the description
/// e.g. [core "description"]
/// returns ("core", Some("description"))
fn get_section_name(s: &str) -> (String, Option<String>) {
    let mut parts = s.splitn(2, ' ');
    let section_name = sanitize(parts.next().unwrap().to_string());
    let description = parts.next().map(|s| sanitize(s.to_string()));

    (section_name, description)
}

#[derive(Debug)]
pub enum OptionValue {
    String(String),
    List(Vec<String>),
}

/// each in the .git/config file
#[derive(Debug)]
pub struct GitConfigSection {
    pub name: String,
    pub description: Option<String>,
    pub options: HashMap<String, OptionValue>,
}

impl GitConfigSection {
    /// get the value of a given key in the section
    pub fn query<T: AsRef<str>>(&self, key: T) -> Option<&OptionValue> {
        self.options.get(key.as_ref())
    }
}

/// set of sections in the .git/config file
#[derive(Debug)]
pub struct GitConfig {
    path: PathBuf,
    sections: Vec<GitConfigSection>,
}

impl GitConfig {
    pub fn new(path: PathBuf, sections: Vec<GitConfigSection>) -> Self {
        Self { path, sections }
    }

    pub fn from_path(path: PathBuf) -> Result<Self> {
        let s = std::fs::read_to_string(&path)?;

        let mut sections = Vec::new();
        let mut lines = s.lines().peekable();

        fn parse_line(line: &str) -> Result<(String, String)> {
            let mut parts = line.splitn(2, '=');
            let key = parts.next().ok_or_eyre("Invalid git config")?.trim().to_string();
            let value = parts.next().ok_or_eyre("Invalid git config")?.trim().to_string();
            Ok((key, value))
        }

        while let Some(line) = lines.next() {
            if line.starts_with('[') {
                let (name, description) = get_section_name(line);
                let mut options = HashMap::new();

                while let Some(line) = lines.peek() {
                    if line.starts_with('[') {
                        break;
                    }

                    let line = lines.next().unwrap();

                    if let Ok((key, value)) = parse_line(line) {
                        // check if the options map already has a value for the key; in that case, we
                        // need to convert the value to a list (OptionValue::List) to keep both values.
                        // Otherwise, we just insert the key-value pair as OptionValue::String

                        if let Some(OptionValue::String(v)) = options.get(&key) {
                            let list = vec![v.clone(), value];
                            options.insert(key, OptionValue::List(list));
                        } else {
                            options.insert(key, OptionValue::String(value));
                        }
                    } else {
                        // if the line is not a key-value pair or we failed to parse it, we skip it
                        continue;
                    }
                }

                sections.push(GitConfigSection {
                    name,
                    description,
                    options,
                });
            }
        }

        Ok(Self::new(path, sections))
    }

    /// get the root path of the repository
    ///
    /// if the config file is in path/to/repo/.git/config, this function will return path/to/repo
    pub fn get_repo_path(&self) -> &Path {
        self.path.parent().unwrap().parent().unwrap()
    }

    /// get all sections matching the given name
    pub fn query<T: AsRef<str>>(&self, name: T) -> Option<Vec<&GitConfigSection>> {
        let key = name.as_ref();
        let result: Vec<&GitConfigSection> =
            self.sections.iter().filter(|s| s.name == key).collect();

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    /// Get the remote URL of the repository
    pub fn get_remote_url_by_filter<T: AsRef<str>>(&self, contains: T) -> Option<String> {
        let filter = contains.as_ref();

        if let Some(remotes) = self.query("remote") {
            for remote in remotes {
                if let Some(value) = remote.query("url") {
                    if let OptionValue::String(url) = value {
                        if url.contains(filter) {
                            return Some(url.clone());
                        }
                    }
                }
            }
        }

        None
    }

    /// Get all remotes of the repository
    pub fn get_remotes(&self) -> Vec<(String, String)> {
        let mut remotes_list = Vec::new();

        if let Some(remotes) = self.query("remote") {
            for remote in remotes {
                if let (Some(name), Some(url)) = (remote.description.clone(), remote.query("url")) {
                    if let OptionValue::String(url) = url {
                        remotes_list.push((name, url.clone()));
                    }
                }
            }
        }

        remotes_list
    }
}
