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

fn git() -> (String, String) {
    #[derive(Default)]
    struct GitFileInfo {
        pub modified: i64,
        pub added: i64,
        pub deleted: i64,
        pub copied: i64,
    }
    impl GitFileInfo {
        fn add(self, chr: &char) -> Self {
            match chr {
                'M' => Self {
                    modified: self.modified + 1,
                    ..self
                },
                'A' => Self {
                    added: self.added + 1,
                    ..self
                },
                'D' => Self {
                    deleted: self.deleted + 1,
                    ..self
                },
                'R' | 'C' => Self {
                    copied: self.copied + 1,
                    ..self
                },
                _ => self,
            }
        }
    }
    #[derive(Default)]
    struct GitInfo {
        pub staged: GitFileInfo,
        pub unstaged: GitFileInfo,
        pub unmerged: i64,
        pub untracked: i64,

        //Branch info
        pub ahead: i64,
        pub behind: i64,
        pub oid: String,
        pub head: String,
        pub upstream: String,
    }
    impl GitInfo {
        fn add(self, line: &str) -> Self {
            let mut words = line.split_whitespace();
            let scan_word = words.next().unwrap_or("_");
            match scan_word.chars().nth(0).unwrap() {
                'f' => Self {
                    head: String::from("FATAL"),
                    ..self
                }, //git failed to get info
                'u' => Self {
                    unmerged: self.unmerged + 1,
                    ..self
                },
                '?' => Self {
                    untracked: self.untracked + 1,
                    ..self
                },
                '1' | '2' => {
                    let chars: Vec<char> = words.next().unwrap().chars().collect();
                    Self {
                        staged: self.staged.add(&chars[0]),
                        unstaged: self.unstaged.add(&chars[1]),
                        ..self
                    }
                }
                '#' => match words.next().unwrap() {
                    "branch.oid" => Self {
                        oid: String::from(words.next().unwrap()),
                        ..self
                    },
                    "branch.head" => Self {
                        head: String::from(words.next().unwrap()),
                        ..self
                    },
                    "branch.upstream" => Self {
                        upstream: String::from(words.next().unwrap()),
                        ..self
                    },
                    "branch.ab" => Self {
                        ahead: words
                            .next()
                            .unwrap()
                            .strip_prefix('+')
                            .unwrap()
                            .parse()
                            .unwrap(),
                        behind: words
                            .next()
                            .unwrap()
                            .strip_prefix('-')
                            .unwrap()
                            .parse()
                            .unwrap(),
                        ..self
                    },
                    _ => self,
                },
                '_' => self,
                _ => self,
            }
        }
    }
    //get output
    let output = std::process::Command::new("git")
        .args(&["status", "--branch", "--porcelain=v2"])
        .output()
        .map(|a| a.stdout)
        .unwrap_or("fatal: git failed to run".bytes().collect());
    let info: GitInfo = String::from_utf8_lossy(&output)
        .lines()
        .fold(GitInfo::default(), |info, line| info.add(line));

    use colorful::Color::{Green, LightGray, Magenta, Red, Yellow};
    use colorful::Colorful;
    use colorful::Style::Bold;
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

fn main() {
    use colorful::{Color, Colorful};

    prints!(toolbox());

    let path_seg = format!("<{}>", path());
    prints!(path_seg.color(Color::Cyan));

    let (branch, diff) = git();
    let branch_seg = format!("<{}>", branch);
    prints!(branch_seg.color(Color::Violet));

    if diff != "" {
        print!("<{}>", diff);
    }

    prints!("-> ".color(arrow_color()));
}
