mod helper_structs;

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
fn git_porcelain_process(porcelain_output: &str) -> (String, String) {
    use colorful::Color::{Green, LightGray, Magenta, Red, Yellow};
    use colorful::Colorful;
    use colorful::Style::Bold;
    use helper_structs::GitInfo;

    let info: GitInfo = porcelain_output
        .lines()
        .fold(GitInfo::default(), |info, line| info.add(line));

    let mut diff_seg = String::new();
    if info.untracked > 0 {
        diff_seg = format!(
            "{}{}",
            diff_seg,
            format!("+{}", info.untracked).color(LightGray)
        );
    }
    if info.staged.added > 0 {
        diff_seg = format!(
            "{}{}",
            diff_seg,
            format!("+{}", info.staged.added).color(Green).style(Bold)
        );
    }
    if info.staged.modified > 0 {
        diff_seg = format!(
            "{}{}",
            diff_seg,
            format!("~{}", info.staged.modified)
                .color(Yellow)
                .style(Bold)
        );
    }
    if info.staged.deleted > 0 {
        diff_seg = format!(
            "{}{}",
            diff_seg,
            format!("-{}", info.staged.deleted).color(Red).style(Bold)
        );
    }
    if info.unstaged.added > 0 {
        diff_seg = format!(
            "{}{}",
            diff_seg,
            format!("+{}", info.unstaged.added).color(Green)
        );
    }
    if info.unstaged.modified > 0 {
        diff_seg = format!(
            "{}{}",
            diff_seg,
            format!("~{}", info.unstaged.modified).color(Yellow)
        );
    }
    if info.unstaged.deleted > 0 {
        diff_seg = format!(
            "{}{}",
            diff_seg,
            format!("-{}", info.unstaged.deleted).color(Red)
        );
    }
    if info.unmerged > 0 {
        diff_seg = format!(
            "{}{}",
            diff_seg,
            format!("!{}", info.unmerged).color(Magenta)
        );
    }
    if info.ahead > 0 {
        diff_seg = format!("{}{}", diff_seg, format!("↑{}", info.ahead))
    }
    if info.behind > 0 {
        diff_seg = format!("{}{}", diff_seg, format!("↓{}", info.behind))
    }
    let branch_seg = if info.head != "" {
        info.head
    } else {
        String::from(info.oid.split_at(8).0)
    };
    (branch_seg, diff_seg)
}
fn git() -> Option<(String, String)> {
    let output = std::process::Command::new("git")
        .args(&["status", "--branch", "--porcelain=v2"])
        .output()
        .map(|a| {
            if a.stderr.len() == 0 {
                a.stdout
            } else {
                a.stderr
            }
        });
    let output = output.unwrap_or("fatal: git failed to run".bytes().collect());
    let output = String::from_utf8(output);
    let output = output.unwrap_or(String::from("fatal: git output was not UTF-8"));
    if output.starts_with("fatal") {
        None
    } else {
        Some(git_porcelain_process(&output))
    }
}

fn main() {
    use colorful::{Color, Colorful};

    prints!(toolbox());

    let path_seg = format!("<{}>", path());
    prints!(path_seg.color(Color::Cyan));

    if let Some((branch, diff)) = git() {
        let branch_seg = format!("<{}>", branch);
        prints!(branch_seg.color(Color::Violet));

        if diff != "" {
            print!("<{}>", diff);
        }
    }

    prints!("-> ".color(arrow_color()));
}
