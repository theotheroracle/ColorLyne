macro_rules! prints {
    ($str:expr) =>(
        print!("{}", $str);
    )
}

fn path() -> String {
    use std::env::{var,current_dir};
    let home_symbol: String = String::from("~");
    let home: String = var("HOME").unwrap_or(home_symbol.clone());
    let cwd: String = current_dir()
        .map(|x|String::from(x.to_string_lossy()))
        .unwrap_or(String::from("unknown"));
    cwd.replace(&home, &home_symbol)
}

fn arrow_color() -> colorful::Color {
    use std::env::var;
    use colorful::Color;
    let status_num: u32 = var("status").map(|x|x.parse().unwrap()).unwrap_or(0);
    if status_num == 0 {
        Color::Green
    }
    else {
        Color::Red
    }
}

fn git() -> Result<String, failure::Error>{
    let repo = git2::Repository::open_from_env()?;
    let head = repo.head()?;
    let head_name = head.name().ok_or(failure::err_msg(""))?;
    Ok(String::from(head_name))
}

fn main() -> Result<(),failure::Error> {
    use colorful::{Colorful, Color};

    let path_seg = format!("<{}>", path());
    prints!(path_seg.color(Color::Cyan));

    let git_val = git()?;
    //if let Ok(git_val) = git(){
        prints!("==");
        let git_seg = format!("<{}>", git_val);
        prints!(git_seg.color(Color::Violet));
    //}

    prints!("->".color(arrow_color()));

    Ok(())
}
