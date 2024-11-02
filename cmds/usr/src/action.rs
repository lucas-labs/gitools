use {
    crate::profiles,
    common::{cli::print, git},
    eyre::Result,
    inquire::{
        required,
        ui::{Attributes, Color, RenderConfig, StyleSheet, Styled},
        Select, Text,
    },
    lool::{cli::stylize::stylize, fail},
    rand::{distributions::Alphanumeric, thread_rng, Rng},
    tabled::{
        settings::{
            object::{Columns, Rows},
            style::HorizontalLine,
            themes::Colorization,
            Alignment, Color as TabledColor, Theme,
        },
        Table,
    },
};

fn generate_random_id(length: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(length).map(char::from).collect()
}

fn inquire_cfg() -> RenderConfig<'static> {
    RenderConfig {
        highlighted_option_prefix: Styled::new(">").with_fg(Color::LightBlue),
        selected_option: Some(
            StyleSheet::new().with_attr(Attributes::ITALIC).with_fg(Color::LightBlue),
        ),
        answer: StyleSheet::new().with_attr(Attributes::ITALIC).with_fg(Color::LightBlue),
        help_message: StyleSheet::new().with_fg(Color::White),
        default_value: StyleSheet::new().with_fg(Color::Rgb {
            r: 50,
            g: 50,
            b: 50,
        }),
        ..Default::default()
    }
}

/// Get the active profile ID from the `gusr.active` git configuration.
/// Executes `git config --get gusr.active` and returns the output.
fn get_active_profile_id() -> Option<String> {
    let output = git::exec("config", vec!["--get", "gusr.active"]);
    return output.ok().map(|o| o.trim().to_string());
}

/// Show the config file content.
pub fn show_config() -> Result<()> {
    let config = profiles::Config::load();
    let content = toml::to_string_pretty(&config);
    if content.is_err() {
        return fail!("Failed to serialize the config file");
    }

    print::bat(&config.path.unwrap(), content.unwrap(), "toml");
    Ok(())
}

/// Show the current active user profile or Fail if none is set.
pub fn view() -> Result<()> {
    let config = profiles::Config::load();
    let active_profile_id = get_active_profile_id();

    if let Some(id) = active_profile_id {
        let profile = config.profile.iter().find(|p| p.id == id);
        if let Some(p) = profile {
            println!("{}{} {}", stylize("Name", "+bold"), stylize(":", "white"), p.name);
            println!("{}{} {}", stylize("Email", "+bold"), stylize(":", "white"), p.email);
            if let Some(key) = &p.signingkey {
                println!("{}{} {}", stylize("Signing key", "+bold"), stylize(":", "white"), key);
            }

            return Ok(());
        }
    }

    fail!(
        "{}\nUse {} to create a new profile and then {} to activate it",
        stylize("No active profile set", "red"),
        stylize("usr add", "blue"),
        stylize("usr set", "blue")
    )
}

/// Print the list of configured user profiles in a table format.
pub fn list() -> Result<()> {
    let config = profiles::Config::load();

    let data = config.profile;
    let len = data.len();

    let mut style = Theme::default();
    let dim = TabledColor::new("\u{1b}[2m", "\u{1b}[22m");

    style.remove_borders_left();
    style.remove_borders_right();
    style.insert_horizontal_line(1, HorizontalLine::full('─', '─', ' ', ' '));
    style.insert_horizontal_line(len + 1, HorizontalLine::full('─', '─', ' ', ' '));
    style.set_colors_horizontal(dim.clone());
    style.set_colors_bottom(dim.clone());
    style.set_colors_top(dim.clone());

    let mut table = Table::new(data);
    table
        .with(style)
        .with(Colorization::exact([TabledColor::BOLD], Columns::first()))
        .with(Colorization::exact([dim], Rows::first()))
        .modify(Rows::first(), Alignment::center());

    println!("{table}");
    Ok(())
}

/// Add a new user profile to the `gusr.toml` file using a series of prompts asking for:
/// - Id (optional) -> Generate a random ID as a default
/// - Name (required)
/// - Email (required)
/// - Signing key (optional)
pub fn add() -> Result<()> {
    inquire::set_global_render_config(inquire_cfg());
    let mut config = profiles::Config::load();

    // generate a random id until it's unique in the config
    let mut random_id;
    loop {
        random_id = generate_random_id(4);
        if !config.profile.iter().any(|p| p.id == random_id) {
            break;
        }
    }

    let id = Text::new("id").with_default(&random_id).prompt_skippable()?;

    let name = Text::new("name").with_validator(required!("Name is required")).prompt()?;
    let email = Text::new("email").with_validator(required!("Email is required")).prompt()?;
    let signingkey = Text::new("key").prompt_skippable()?;

    let id = id.unwrap_or_else(|| generate_random_id(8));
    let profile = profiles::Profile {
        id,
        name,
        email,
        signingkey: signingkey.filter(|s| !s.is_empty()),
    };

    config.profile.push(profile);
    config.save();
    Ok(())
}

/// Remove a user profile from the `gusr.toml` file using a Select prompt to choose the profile to
/// remove.
pub fn remove() -> Result<()> {
    let mut config = profiles::Config::load();
    let options = config.profile.iter().map(|p| p.id.clone()).collect::<Vec<_>>();

    let ans = Select::new("Select a profile to remove", options)
        .with_page_size(10)
        .without_help_message()
        .with_render_config(inquire_cfg())
        .prompt();

    match ans {
        Ok(choice) => {
            config.profile.retain(|p| p.id != choice);
            config.save();
            Ok(())
        }
        Err(err) => fail!("{}", stylize(err.to_string(), "red")),
    }
}

/// Set the active user profile by choosing from a list of profiles.
/// The selected profile is then set as the active profile by setting the `gusr.active` git
/// configuration globally.
///
/// Also sets the user.name, user.email, and user.signingkey to match the selected profile.
pub fn set() -> Result<()> {
    let config = profiles::Config::load();
    let options = config.profile.iter().map(|p| p.id.clone()).collect::<Vec<_>>();

    let ans = Select::new("Select a profile to set as active", options)
        .with_page_size(10)
        .without_help_message()
        .with_render_config(inquire_cfg())
        .prompt();

    match ans {
        Ok(choice) => {
            let profile = config.profile.iter().find(|p| p.id == choice);

            match profile {
                Some(p) => {
                    git::exec("config", vec!["--global", "gusr.active", &p.id])?;
                    git::exec("config", vec!["--global", "user.name", &p.name])?;
                    git::exec("config", vec!["--global", "user.email", &p.email])?;
                    if let Some(key) = &p.signingkey {
                        git::exec("config", vec!["--global", "user.signingkey", key])?;
                    } else {
                        git::exec("config", vec!["--global", "--unset", "user.signingkey"])?;
                    }

                    println!("Active profile set to: {} <{}>", p.name, p.email);
                }
                None => {
                    return fail!(
                        "{}\n{}",
                        stylize("Profile not found", "red"),
                        stylize("Try `usr list` to see all profiles", "yellow")
                    );
                }
            }

            Ok(())
        }
        Err(err) => fail!("{}", stylize(err.to_string(), "red")),
    }
}
