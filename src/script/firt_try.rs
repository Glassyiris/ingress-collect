

fn try_ls_root() {
    // let mut out = spawn_with_output!(
    //    ls /home/eries/
    // )?.wait_result();
    //
    // println!("{}", out)
}

#[cfg(test)]
mod test {
    use std::process::Command;
    use crate::script::operate::preprocess;

    #[test]
    fn test() -> std::io::Result<()> {
        preprocess()
    }
}