use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

#[inline]
pub fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn eval_choice(choices: i32, new_line: bool) -> char {
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim_end().len() == 0 && new_line {
                    return '\n';
                }
                if input.trim_end().len() != 1 {
                    println!("invalid input");
                    continue;
                }
                match input.as_bytes()[0] as char {
                    c @ 'a'..='z' => {
                        let i = input.as_bytes()[0] - 'a' as u8;
                        assert!((i as i32) < choices, "option out of range");
                        return c;
                    }
                    _ => {
                        println!("invalid input");
                        continue;
                    }
                };
            }
            Err(error) => println!("error: {error}"),
        }
    }
}

pub fn yes_or_no() -> char {
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim_end().len() != 1 {
                    println!("invalid input");
                    continue;
                }
                match input.as_bytes()[0] as char {
                    c @ ('y' | 'n') => return c,
                    _ => {
                        println!("invalid input");
                        continue;
                    }
                };
            }
            Err(error) => println!("error: {error}"),
        }
    }
}

fn execute_as(script: String) -> String {
    let dir = script_file(script.as_str());

    let mut child = Command::new("osascript")
        .arg(dir.to_str().unwrap())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    match child.wait() {
        Ok(status) => {
            if !status.success() {
                println!("\u{001b}[43;1m CANCEL \u{001b}[0m");
                std::process::exit(0);
            }
        }
        Err(_) => panic!("command osascript wasn't running"),
    }

    let mut stdout = child.stdout.take().unwrap();
    let mut answer = String::new();
    stdout
        .read_to_string(&mut answer)
        .expect("failed to read from child's stdout");
    answer.trim_end().to_string()
}

pub fn script_file(script: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    dir.push("rt_temp");
    if !dir.exists() {
        fs::create_dir(&dir).expect("failed to create temp folder");
    }
    let rnd_name = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    dir.push(format!("rand-task-{rnd_name}.osas"));
    fs::File::create(&dir).expect("failed to create temp file");
    fs::write(&dir, script).expect("failed to write to temp file");
    dir
}

pub fn get_dialog_answer(title: &str, default: &str) -> String {
    let script = format!(
        r#"on run argv
	set answer to display dialog "{title}" default answer "{default}"
	set content to text returned of answer
end run"#
    );
    execute_as(script)
}

pub fn choose_from_list(title: &str, list: Vec<&str>, default: Vec<i32>) -> Vec<String> {
    assert!(list.len() > 0);
    assert!(default.len() > 0);
    assert!(default.len() <= list.len());
    let mut list_str = String::with_capacity(64);
    let mut list_iter = list.iter();
    list_str.push('"');
    list_str.push_str(list_iter.next().unwrap());
    list_str.push('"');
    for i in list_iter {
        list_str.push_str(r#", ""#);
        list_str.push_str(i);
        list_str.push('"');
    }
    let mut default_str = String::with_capacity(64);
    let mut default_iter = default.iter();
    default_str.push_str("item ");
    default_str.push_str(&*((default_iter.next().unwrap() + 1).to_string()));
    default_str.push_str(" of serverList");
    for i in default_iter {
        default_str.push_str(", item ");
        default_str.push_str(&*((i + 1).to_string()));
        default_str.push_str(" of serverList");
    }

    let script = format!(
        r#"on run argv
	set serverList to {{{list_str}}}
	set serverChoice to (choose from list serverList with prompt "{title}" default items {{{default_str}}} OK button name "Ok" cancel button name "Cancel" with multiple selections allowed)
end run
"#
    );
    let rst = execute_as(script);
    rst.split(",")
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>()
}

#[inline]
pub fn is_rt_md(file: Option<&String>) -> bool {
    file.is_some() && file.unwrap().ends_with(".md") && !file.unwrap().contains("/")
}

#[cfg(test)]
mod test {
    #[test]
    fn test_get_dialog_answer() {
        dbg!(super::get_dialog_answer("xx", "bb"));
    }

    #[test]
    fn test_choose_from_list() {
        dbg!(super::choose_from_list(
            "xx",
            vec!["a", "b", "c"],
            vec![0, 1, 2]
        ));
    }
}
