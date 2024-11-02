use {
    serde::{Deserialize, Serialize},
    std::{borrow::Cow, fs, path::PathBuf, process},
    tabled::Tabled,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub email: String,
    pub signingkey: Option<String>,
}

impl Tabled for Profile {
    const LENGTH: usize = 4;

    fn fields(&self) -> Vec<Cow<'_, str>> {
        vec![
            Cow::from(self.id.as_str()),
            Cow::from(self.name.as_str()),
            Cow::from(self.email.as_str()),
            Cow::from(self.signingkey.as_deref().unwrap_or("None")),
        ]
    }

    fn headers() -> Vec<Cow<'static, str>> {
        vec![Cow::from("id"), Cow::from("name"), Cow::from("email"), Cow::from("gpg")]
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[serde(skip)]
    pub path: Option<String>,
    pub profile: Vec<Profile>,
}

impl Config {
    pub fn load() -> Config {
        let path = get_profiles_path();
        if !path.exists() {
            let profiles = Config {
                path: Some(String::from(path.to_str().unwrap())),
                profile: Vec::new(),
            };
            write_profiles(&profiles);
            return profiles;
        }

        let content = fs::read_to_string(&path).unwrap_or_else(|_| {
            eprintln!("Could not read the profiles file.");
            process::exit(1);
        });

        let mut cfg: Config = toml::from_str(&content).unwrap_or_else(|_| {
            eprintln!("Could not parse the profiles file.");
            process::exit(1);
        });

        cfg.path = Some(String::from(path.to_str().unwrap()));
        cfg
    }

    pub fn save(&self) {
        write_profiles(self);
    }
}

/// Read the profiles from the `gusr.toml` file, located in the same directory as the executable.
/// If the file does not exist, it will create it with an empty list of profiles.

fn get_profiles_path() -> PathBuf {
    let mut path = std::env::current_exe().unwrap_or_else(|_| {
        eprintln!("Could not get the current executable path.");
        process::exit(1);
    });
    path.pop();
    path.push("gusr.toml");
    path
}

fn write_profiles(profiles: &Config) {
    let path = get_profiles_path();
    let content = toml::to_string_pretty(profiles).unwrap_or_else(|_| {
        eprintln!("Could not serialize the profiles.");
        process::exit(1);
    });

    fs::write(path, content).unwrap_or_else(|_| {
        eprintln!("Could not write the profiles file.");
        process::exit(1);
    });
}
