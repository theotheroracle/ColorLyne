#[derive(Default)]
pub struct GitFileInfo {
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
pub struct GitInfo {
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
    pub fn add(self, line: &str) -> Self {
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
