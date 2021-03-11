use std::process::Command;
use std::string::FromUtf8Error;

pub fn preprocess() -> std::io::Result<()> {
    let mut output = Command::new("ls")
        .arg("-al")
        .output()?;

    let text = String::from_utf8(output.stdout).unwrap();

    let mut collector = text.lines().map(|x|
        x.split_whitespace().collect::<Vec<&str>>()
    ).collect::<Vec<Vec<&str>>>();
    collector.remove(0);
    println!("{:?}", collector);
    Ok(())
}

pub fn parse_domain(url: &str) -> Vec<String> {
    url.split(',').map(|x|x.to_string()).collect()
}

pub fn save() {}
#[cfg(test)]
mod test {
    use crate::script::operate::preprocess;

    #[test]
    fn test() -> std::io::Result<()> {
        preprocess()
    }
}