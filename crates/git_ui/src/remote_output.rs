use anyhow::Context as _;

use git::repository::{Remote, RemoteCommandOutput};
use linkify::{LinkFinder, LinkKind};
use localization::UiLanguage;
use ui::SharedString;
use util::ResultExt as _;

#[derive(Clone)]
pub enum RemoteAction {
    Fetch(Option<Remote>),
    Pull(Remote),
    Push(SharedString, Remote),
}

impl RemoteAction {
    pub fn name(&self) -> &'static str {
        match self {
            RemoteAction::Fetch(_) => "fetch",
            RemoteAction::Pull(_) => "pull",
            RemoteAction::Push(_, _) => "push",
        }
    }
}

pub enum SuccessStyle {
    Toast,
    ToastWithLog { output: RemoteCommandOutput },
    PushPrLink { text: String, link: String },
}

pub struct SuccessMessage {
    pub message: String,
    pub style: SuccessStyle,
}

#[cfg(test)]
pub fn format_output(action: &RemoteAction, output: RemoteCommandOutput) -> SuccessMessage {
    format_output_for_language(action, output, UiLanguage::English)
}

pub fn format_output_for_language(
    action: &RemoteAction,
    output: RemoteCommandOutput,
    language: UiLanguage,
) -> SuccessMessage {
    match action {
        RemoteAction::Fetch(remote) => {
            if output.stderr.is_empty() {
                SuccessMessage {
                    message: match language {
                        UiLanguage::ChineseSimplified => "获取：已经是最新".into(),
                        UiLanguage::English => "Fetch: Already up to date".into(),
                    },
                    style: SuccessStyle::Toast,
                }
            } else {
                let message = match (remote, language) {
                    (Some(remote), UiLanguage::ChineseSimplified) => {
                        format!("已与 {} 同步", remote.name)
                    }
                    (Some(remote), UiLanguage::English) => {
                        format!("Synchronized with {}", remote.name)
                    }
                    (None, UiLanguage::ChineseSimplified) => "已与远程同步".into(),
                    (None, UiLanguage::English) => "Synchronized with remotes".into(),
                };
                SuccessMessage {
                    message,
                    style: SuccessStyle::ToastWithLog { output },
                }
            }
        }
        RemoteAction::Pull(remote_ref) => {
            let get_changes = |output: &RemoteCommandOutput| -> anyhow::Result<u32> {
                let last_line = output
                    .stdout
                    .lines()
                    .last()
                    .context("Failed to get last line of output")?
                    .trim();

                let files_changed = last_line
                    .split_whitespace()
                    .next()
                    .context("Failed to get first word of last line")?
                    .parse()?;

                Ok(files_changed)
            };
            if output.stdout.ends_with("Already up to date.\n") {
                SuccessMessage {
                    message: match language {
                        UiLanguage::ChineseSimplified => "拉取：已经是最新".into(),
                        UiLanguage::English => "Pull: Already up to date".into(),
                    },
                    style: SuccessStyle::Toast,
                }
            } else if output.stdout.starts_with("Updating") {
                let files_changed = get_changes(&output).log_err();
                let message = match (files_changed, language) {
                    (Some(files_changed), UiLanguage::ChineseSimplified) => {
                        format!("从 {} 接收了 {} 个文件变更", remote_ref.name, files_changed)
                    }
                    (Some(files_changed), UiLanguage::English) => format!(
                        "Received {} file change{} from {}",
                        files_changed,
                        if files_changed == 1 { "" } else { "s" },
                        remote_ref.name
                    ),
                    (None, UiLanguage::ChineseSimplified) => {
                        format!("已从 {} 快进", remote_ref.name)
                    }
                    (None, UiLanguage::English) => {
                        format!("Fast forwarded from {}", remote_ref.name)
                    }
                };
                SuccessMessage {
                    message,
                    style: SuccessStyle::ToastWithLog { output },
                }
            } else if output.stdout.starts_with("Merge") {
                let files_changed = get_changes(&output).log_err();
                let message = match (files_changed, language) {
                    (Some(files_changed), UiLanguage::ChineseSimplified) => {
                        format!("从 {} 合并了 {} 个文件变更", remote_ref.name, files_changed)
                    }
                    (Some(files_changed), UiLanguage::English) => format!(
                        "Merged {} file change{} from {}",
                        files_changed,
                        if files_changed == 1 { "" } else { "s" },
                        remote_ref.name
                    ),
                    (None, UiLanguage::ChineseSimplified) => {
                        format!("已从 {} 合并", remote_ref.name)
                    }
                    (None, UiLanguage::English) => format!("Merged from {}", remote_ref.name),
                };
                SuccessMessage {
                    message,
                    style: SuccessStyle::ToastWithLog { output },
                }
            } else if output.stdout.contains("Successfully rebased") {
                SuccessMessage {
                    message: match language {
                        UiLanguage::ChineseSimplified => {
                            format!("已从 {} 变基", remote_ref.name)
                        }
                        UiLanguage::English => {
                            format!("Successfully rebased from {}", remote_ref.name)
                        }
                    },
                    style: SuccessStyle::ToastWithLog { output },
                }
            } else {
                SuccessMessage {
                    message: match language {
                        UiLanguage::ChineseSimplified => {
                            format!("已从 {} 拉取", remote_ref.name)
                        }
                        UiLanguage::English => {
                            format!("Successfully pulled from {}", remote_ref.name)
                        }
                    },
                    style: SuccessStyle::ToastWithLog { output },
                }
            }
        }
        RemoteAction::Push(branch_name, remote_ref) => {
            let message = match (output.stderr.ends_with("Everything up-to-date\n"), language) {
                (true, UiLanguage::ChineseSimplified) => "推送：全部已是最新".to_string(),
                (true, UiLanguage::English) => "Push: Everything is up-to-date".to_string(),
                (false, UiLanguage::ChineseSimplified) => {
                    format!("已将 {} 推送到 {}", branch_name, remote_ref.name)
                }
                (false, UiLanguage::English) => {
                    format!("Pushed {} to {}", branch_name, remote_ref.name)
                }
            };

            let style = if output.stderr.ends_with("Everything up-to-date\n") {
                Some(SuccessStyle::Toast)
            } else if output.stderr.contains("\nremote: ") {
                let pr_hints = [
                    (
                        "Create a pull request",
                        "Create Pull Request",
                        "创建 Pull Request",
                    ), // GitHub
                    (
                        "Create pull request",
                        "Create Pull Request",
                        "创建 Pull Request",
                    ), // Bitbucket
                    (
                        "create a merge request",
                        "Create Merge Request",
                        "创建合并请求",
                    ), // GitLab
                    ("View merge request", "View Merge Request", "查看合并请求"), // GitLab
                ];
                pr_hints
                    .iter()
                    .find(|(indicator, _, _)| output.stderr.contains(indicator))
                    .and_then(|(_, mapped, mapped_zh)| {
                        let finder = LinkFinder::new();

                        output
                            .stderr
                            .lines()
                            .filter(|line| line.trim_start().starts_with("remote:"))
                            .find_map(|line| {
                                finder
                                    .links(line)
                                    .find(|link| *link.kind() == LinkKind::Url)
                                    .map(|link| SuccessStyle::PushPrLink {
                                        text: match language {
                                            UiLanguage::ChineseSimplified => mapped_zh.to_string(),
                                            UiLanguage::English => mapped.to_string(),
                                        },
                                        link: link.as_str().to_string(),
                                    })
                            })
                    })
            } else {
                None
            };
            SuccessMessage {
                message,
                style: style.unwrap_or(SuccessStyle::ToastWithLog { output }),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_push_new_branch_pull_request() {
        let action = RemoteAction::Push(
            SharedString::new_static("test_branch"),
            Remote {
                name: SharedString::new_static("test_remote"),
            },
        );

        let output = RemoteCommandOutput {
            stdout: String::new(),
            stderr: indoc! { "
                Total 0 (delta 0), reused 0 (delta 0), pack-reused 0 (from 0)
                remote:
                remote: Create a pull request for 'test' on GitHub by visiting:
                remote:      https://example.com/test/test/pull/new/test
                remote:
                To example.com:test/test.git
                 * [new branch]      test -> test
                "}
            .to_string(),
        };

        let msg = format_output(&action, output);

        if let SuccessStyle::PushPrLink { text: hint, link } = &msg.style {
            assert_eq!(hint, "Create Pull Request");
            assert_eq!(link, "https://example.com/test/test/pull/new/test");
        } else {
            panic!("Expected PushPrLink variant");
        }
    }

    #[test]
    fn test_push_new_branch_merge_request() {
        let action = RemoteAction::Push(
            SharedString::new_static("test_branch"),
            Remote {
                name: SharedString::new_static("test_remote"),
            },
        );

        let output = RemoteCommandOutput {
            stdout: String::new(),
            stderr: indoc! {"
                Total 0 (delta 0), reused 0 (delta 0), pack-reused 0 (from 0)
                remote:
                remote: To create a merge request for test, visit:
                remote:   https://example.com/test/test/-/merge_requests/new?merge_request%5Bsource_branch%5D=test
                remote:
                To example.com:test/test.git
                 * [new branch]      test -> test
                "}
            .to_string()
            };

        let msg = format_output(&action, output);

        if let SuccessStyle::PushPrLink { text, link } = &msg.style {
            assert_eq!(text, "Create Merge Request");
            assert_eq!(
                link,
                "https://example.com/test/test/-/merge_requests/new?merge_request%5Bsource_branch%5D=test"
            );
        } else {
            panic!("Expected PushPrLink variant");
        }
    }

    #[test]
    fn test_push_branch_existing_merge_request() {
        let action = RemoteAction::Push(
            SharedString::new_static("test_branch"),
            Remote {
                name: SharedString::new_static("test_remote"),
            },
        );

        let output = RemoteCommandOutput {
            stdout: String::new(),
            // Simulate an extraneous link that should not be found in top 3 lines
            stderr: indoc! {"
                ** WARNING: connection is not using a post-quantum key exchange algorithm.
                ** This session may be vulnerable to \"store now, decrypt later\" attacks.
                ** The server may need to be upgraded. See https://openssh.com/pq.html
                Total 0 (delta 0), reused 0 (delta 0), pack-reused 0 (from 0)
                remote:
                remote: View merge request for test:
                remote:    https://example.com/test/test/-/merge_requests/99999
                remote:
                To example.com:test/test.git
                    + 80bd3c83be...e03d499d2e test -> test
                "}
            .to_string(),
        };

        let msg = format_output(&action, output);

        if let SuccessStyle::PushPrLink { text, link } = &msg.style {
            assert_eq!(text, "View Merge Request");
            assert_eq!(link, "https://example.com/test/test/-/merge_requests/99999");
        } else {
            panic!("Expected PushPrLink variant");
        }
    }

    #[test]
    fn test_push_new_branch_no_link() {
        let action = RemoteAction::Push(
            SharedString::new_static("test_branch"),
            Remote {
                name: SharedString::new_static("test_remote"),
            },
        );

        let output = RemoteCommandOutput {
            stdout: String::new(),
            stderr: indoc! { "
                To http://example.com/test/test.git
                 * [new branch]      test -> test
                ",
            }
            .to_string(),
        };

        let msg = format_output(&action, output);

        if let SuccessStyle::ToastWithLog { output } = &msg.style {
            assert_eq!(
                output.stderr,
                "To http://example.com/test/test.git\n * [new branch]      test -> test\n"
            );
        } else {
            panic!("Expected ToastWithLog variant");
        }
    }
}
