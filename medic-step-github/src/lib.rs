#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use cli::app::GithubActionsArgs;
use medic_lib::std_to_string;
use medic_lib::StepResult::{self, StepError, StepOk};
use regex::Regex;

use std::process::Command;

pub fn link_to_actions(args: GithubActionsArgs) -> StepResult {
    match Command::new("git")
        .args(["remote", "get-url", &args.remote])
        .output()
    {
        Ok(cmd) => {
            if cmd.status.success() {
                let mut origin = std_to_string(cmd.stdout);
                origin = origin.trim().to_owned();
                let github_url = origin_to_github_url(origin);

                println!("\x1b[93mCheck CI at \x1b[0;1m{github_url}/actions\x1b[0m");

                StepOk
            } else {
                let stdout = std_to_string(cmd.stdout);
                let stderr = std_to_string(cmd.stderr);

                StepError("Git remote error".into(), Some(stdout), Some(stderr))
            }
        }
        Err(_err) => StepError(
            "Could not run `git remote get-url`. Is `git` in PATH?".into(),
            None,
            None,
        ),
    }
}

pub fn origin_to_github_url(origin: String) -> String {
    let trailing_git_re = Regex::new(r"\.git$").unwrap();
    let origin = trailing_git_re.replace_all(&origin, "");

    let origin_re = Regex::new(r"^([^@]+@|[^:]+://)(ssh.)?([^:]+)(:443)?[:/](.+)$").unwrap();
    let caps = origin_re.captures(&origin).unwrap();
    let url = caps.get(3).unwrap().as_str();
    let repository = caps.get(5).unwrap().as_str();

    format!("https://{url}/{repository}")
}

#[cfg(test)]
mod tests {
    #[test]
    fn github_actions_url_from_ssh() {
        let url = super::origin_to_github_url("git@github.com:my-org/my-repo".to_owned());
        assert_eq!(url, "https://github.com/my-org/my-repo")
    }

    #[test]
    fn github_actions_url_from_ssh_with_trailing_git() {
        let url = super::origin_to_github_url("git@github.com:my-org/my-repo.git".to_owned());
        assert_eq!(url, "https://github.com/my-org/my-repo")
    }

    #[test]
    fn github_actions_url_from_https() {
        let url = super::origin_to_github_url("https://github.com/my-org/my-repo".to_owned());
        assert_eq!(url, "https://github.com/my-org/my-repo")
    }

    #[test]
    fn github_actions_url_from_https_ssh() {
        let url =
            super::origin_to_github_url("https://ssh.github.com:443/my-org/my-repo".to_owned());
        assert_eq!(url, "https://github.com/my-org/my-repo")
    }

    #[test]
    fn github_actions_url_from_https_with_trailing_git() {
        let url = super::origin_to_github_url("https://github.com/my-org/my-repo.git".to_owned());
        assert_eq!(url, "https://github.com/my-org/my-repo")
    }

    #[test]
    fn github_actions_url_from_git() {
        let url = super::origin_to_github_url("git://github.com/my-org/my-repo".to_owned());
        assert_eq!(url, "https://github.com/my-org/my-repo")
    }

    #[test]
    fn github_actions_url_from_git_with_trailing_git() {
        let url = super::origin_to_github_url("git://github.com/my-org/my-repo.git".to_owned());
        assert_eq!(url, "https://github.com/my-org/my-repo")
    }
}
