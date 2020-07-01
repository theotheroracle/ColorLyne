#[warn(clippy::pedantic)]
#[warn(clippy::nursery)]
#[warn(clippy::cargo)]
macro_rules! prints {
    ($str:expr) => {
        print!("{}", $str);
    };
}

fn toolbox() -> String {
    use std::env::var;
    if let Ok(hostname) = var("HOSTNAME") {
        if hostname == "toolbox" {
            return String::from("⚙ ");
        }
    }
    String::default()
}

fn path() -> String {
    use std::env::{current_dir, var};
    let cwd: String = current_dir()
        .map(|x| String::from(x.to_string_lossy()))
        .unwrap_or_else(|_| String::from("unknown"));

    if let Ok(home) = var("HOME") {
        cwd.replace(&home, "~")
    } else {
        cwd
    }
}

fn arrow_color() -> colorful::Color {
    use colorful::Color;
    use std::env::var;
    let status_num: u32 = var("status").map(|x| x.parse().unwrap()).unwrap_or(0);
    if status_num == 0 {
        Color::Green
    } else {
        Color::Red
    }
}

fn git() -> Result<(String, String), git2::Error> {
    let repo = git2::Repository::open_from_env()?;
    fn git_commit_id(commit: git2::Commit) -> String {
        commit
            .id()
            .as_bytes()
            .iter()
            .map(|x| format!("{:02x}", x))
            .fold(String::new(), |acc, x| acc + &x)
            .get(..7)
            .unwrap_or_else(|| "UNKNOWN")
            .into()
    }
    fn git_branch_seg(repo: &git2::Repository) -> Result<String, git2::Error> {
        let head = repo.head()?;
        let pretty_name = if let Some(head_name) = head.shorthand() {
            if head_name == "HEAD" {
                //We're *somewhere* so let's just put the short id
                git_commit_id(head.peel_to_commit()?)
            } else {
                String::from(head_name)
            }
        } else {
            String::from("UNKNOWN")
        };
        Ok(pretty_name)
    }
    fn git_diff_seg(repo: &git2::Repository) -> Result<String, git2::Error> {
        use colorful::Color::{Green, Magenta, Red, Yellow};
        use colorful::Colorful;
        use colorful::Style::Bold;
        use git2::Status;
        let ((inew, imod, idel), (wnew, wmod, wdel), conf) = repo
            .statuses(Some(
                git2::StatusOptions::new()
                    .update_index(true)
                    .include_untracked(true),
            ))?
            .iter()
            .fold(
                ((0, 0, 0), (0, 0, 0), 0),
                |((inew, imod, idel), (wnew, wmod, wdel), conf), x| match x.status() {
                    Status::INDEX_NEW => ((inew + 1, imod, idel), (wnew, wmod, wdel), conf),
                    Status::INDEX_MODIFIED => ((inew, imod + 1, idel), (wnew, wmod, wdel), conf),
                    Status::INDEX_RENAMED => ((inew, imod + 1, idel), (wnew, wmod, wdel), conf),
                    Status::INDEX_TYPECHANGE => ((inew, imod + 1, idel), (wnew, wmod, wdel), conf),
                    Status::INDEX_DELETED => ((inew, imod, idel + 1), (wnew, wmod, wdel), conf),
                    Status::WT_NEW => ((inew, imod, idel), (wnew + 1, wmod, wdel), conf),
                    Status::WT_MODIFIED => ((inew, imod, idel), (wnew, wmod + 1, wdel), conf),
                    Status::WT_RENAMED => ((inew, imod, idel), (wnew, wmod + 1, wdel), conf),
                    Status::WT_TYPECHANGE => ((inew, imod, idel), (wnew, wmod + 1, wdel), conf),
                    Status::WT_DELETED => ((inew, imod, idel), (wnew, wmod, wdel + 1), conf),
                    Status::CONFLICTED => ((inew, imod, idel), (wnew, wmod, wdel), conf + 1),
                    _ => ((inew, imod, idel), (wnew, wmod, wdel), conf),
                },
            );

        let mut diff = String::new();
        if inew > 0 {
            diff = format!("{}{}", diff, format!("+{}", inew).color(Green).style(Bold));
        }
        if imod > 0 {
            diff = format!("{}{}", diff, format!("~{}", imod).color(Yellow).style(Bold));
        }
        if idel > 0 {
            diff = format!("{}{}", diff, format!("-{}", idel).color(Red).style(Bold));
        }
        if wnew > 0 {
            diff = format!("{}{}", diff, format!("+{}", wnew).color(Green));
        }
        if wmod > 0 {
            diff = format!("{}{}", diff, format!("~{}", wmod).color(Yellow));
        }
        if wdel > 0 {
            diff = format!("{}{}", diff, format!("-{}", wdel).color(Red));
        }
        if conf > 0 {
            diff = format!("{}{}", diff, format!("!{}", conf).color(Magenta));
        }
        Ok(diff)
    }
    Ok((
        git_branch_seg(&repo).unwrap_or_else(|_| String::from("UNKNOWN")),
        git_diff_seg(&repo).unwrap_or_default(),
    ))
}

fn main() {
    use colorful::{Color, Colorful};

    prints!(toolbox());

    let path_seg = format!("<{}>", path());
    prints!(path_seg.color(Color::Cyan));

    if let Ok((branch, diff)) = git() {
        let branch_seg = format!("<{}>", branch);
        prints!(branch_seg.color(Color::Violet));

        if diff != "" {
            print!("<{}>", diff);
        }
    }

    prints!("-> ".color(arrow_color()));
}
