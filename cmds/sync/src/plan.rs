use {
    eyre::Result,
    lool::{cli::stylize::stylize, fail, s},
};

/// A plan of actions to be executed a list of (git command, its args)
pub struct Plan(Vec<(String, Vec<String>)>);

impl Plan {
    /// Create a new plan from arguments
    pub fn from_args<T: Into<String>>(
        args: Vec<String>,
        default_branch: T,
        remotes: Option<Vec<(String, String)>>,
    ) -> Result<Self> {
        let mut commands = Vec::new();

        // Assume we are always in the master branch unless a different branch is specified
        let mut branch = default_branch.into();
        let mut remote_from = String::new();
        let mut remote_to = None;

        for (i, arg) in args.iter().enumerate() {
            if i == 1 {
                // Parse remote:branch if provided
                let parts: Vec<&str> = arg.split(':').collect();
                remote_from = s!(parts[0]);
                if parts.len() > 1 {
                    branch = s!(parts[1]);
                }
            } else if arg == "to" && i + 1 < args.len() {
                // Handle the 'to' part
                remote_to = Some(args[i + 1].clone());
            }
        }

        if let Some(remotes) = remotes {
            // make sure the remotes_to and remotes_from are valid if we have them
            if !remotes.iter().any(|(name, _)| name == &remote_from) {
                return fail!(
                    "Remote '{remote_from}' not found\nRun {} to add the remote",
                    stylize(format!("git remote add {remote_from} <url>"), "bright-blue"),
                );
            }
            if let Some(ref remote) = remote_to {
                if !remotes.iter().any(|(name, _)| name == remote) {
                    return fail!(
                        "Remote '{remote}' not found\nRun {} to add the remote",
                        stylize(format!("git remote add {remote} <url>"), "bright-blue"),
                    );
                }
            }
        }

        // Fetch the changes from the remote
        if branch == "master" {
            commands.push((s!("fetch"), vec![remote_from.clone()]));
        } else {
            commands.push((s!("fetch"), vec![remote_from.clone(), branch.clone()]));
        }

        // If branch isn't master, checkout to the branch and rebase
        if branch != "master" {
            commands.push((s!("checkout"), vec![branch.clone()]));
        }

        // Rebase the changes from the remote branch
        commands.push((s!("rebase"), vec![format!("{}/{}", remote_from, branch)]));

        // If we have a destination remote ('to'), push the changes
        if let Some(remote) = remote_to {
            commands.push((s!("push"), vec![remote, branch]));
        }

        Ok(Plan(commands))
    }

    pub fn print(&self, prefix: Option<&str>) {
        for (cmd, args) in &self.0 {
            println!(
                "{}{} {} {}",
                prefix.unwrap_or(""),
                stylize("git", "white+dim"),
                stylize(cmd, "magenta+bold"),
                stylize(args.join(" "), "bright-blue")
            );
        }
    }

    pub fn commands(&self) -> &Vec<(String, Vec<String>)> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use lool::s;

    use super::Plan;

    fn assert_plan(commands: &Plan, expected: Vec<(&str, Vec<&str>)>) {
        let plan_commands: Vec<(String, Vec<String>)> = commands.0.clone();
        let expected_commands: Vec<(String, Vec<String>)> = expected
            .into_iter()
            .map(|(cmd, args)| (cmd.to_string(), args.into_iter().map(|s| s.to_string()).collect()))
            .collect();
        assert_eq!(plan_commands, expected_commands);
    }
    #[test]
    fn test_sync_from_gitea() {
        let args = vec![s!("sync"), s!("gitea")];
        let plan = Plan::from_args(args, "master", None).unwrap();

        let expected =
            vec![("git", vec!["fetch", "gitea"]), ("git", vec!["rebase", "gitea/master"])];

        assert_plan(&plan, expected);
    }

    #[test]
    fn test_sync_from_gitea_to_github() {
        let args =
            vec!["sync".to_string(), "gitea".to_string(), "to".to_string(), "github".to_string()];
        let plan = Plan::from_args(args, "master", None).unwrap();

        let expected = vec![
            ("git", vec!["fetch", "gitea"]),
            ("git", vec!["rebase", "gitea/master"]),
            ("git", vec!["push", "github", "master"]),
        ];

        assert_plan(&plan, expected);
    }

    #[test]
    fn test_sync_from_github_feature_branch() {
        let args = vec!["sync".to_string(), "github:feature-branch".to_string()];
        let plan = Plan::from_args(args, "master", None).unwrap();

        let expected = vec![
            ("git", vec!["fetch", "github", "feature-branch"]),
            ("git", vec!["checkout", "feature-branch"]),
            ("git", vec!["rebase", "github/feature-branch"]),
        ];

        assert_plan(&plan, expected);
    }

    #[test]
    fn test_sync_from_github_feature_branch_to_gitea() {
        let args = vec![
            "sync".to_string(),
            "github:feature-branch".to_string(),
            "to".to_string(),
            "gitea".to_string(),
        ];
        let plan = Plan::from_args(args, "master", None).unwrap();

        let expected = vec![
            ("git", vec!["fetch", "github", "feature-branch"]),
            ("git", vec!["checkout", "feature-branch"]),
            ("git", vec!["rebase", "github/feature-branch"]),
            ("git", vec!["push", "gitea", "feature-branch"]),
        ];

        assert_plan(&plan, expected);
    }
}
