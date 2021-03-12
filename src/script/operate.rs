use cmd_lib::*;
use std::intrinsics::rustc_peek;

pub fn preprocess() -> CmdResult {
    cmd_lib::set_debug(true); // to print commands
    cmd_lib::set_pipefail(false);
    let mut text: String = spawn_with_output!( kubectl get ing -A )?.wait_result().unwrap();

    let mut collector = text.lines().map(|x|
        x.split_whitespace().collect::<Vec<&str>>()
    ).collect::<Vec<Vec<&str>>>();
    collector.remove(0);
    println!("{:?}", collector);

    Ok(())
}

pub fn get_yaml(t: Vec<&str>) -> CmdResult {
    cmd_lib::set_debug(true); // to print commands
    cmd_lib::set_pipefail(false);
    let namespace = t[0];
    let name = t[1];
    let host = parse_domain(t[2]);
    let mut yaml_pre: String = spawn_with_output!( kubectl -n $namespace get ing $name -oyaml)?.wait_result().unwrap();

    for domain in &host {
        save(domain, &yaml_pre, name, namespace);
    }
    Ok(())
}

pub fn parse_domain(url: &str) -> Vec<String> {
    url.split(',').map(|x| x.to_string()).collect()
}

pub fn save(dirname: &str , data: &str, filename: &str, space: &str) -> CmdResult{
    cmd_lib::set_debug(true); // to print commands
    cmd_lib::set_pipefail(false);
    run_cmd!( mkdir $dirname )?;
    let last_name = dirname + "/" + filename + "+" + space + ".yaml";
    run_cmd!( echo $data > $last_name)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::script::operate::preprocess;
    use cmd_lib::*;

    #[test]
    fn test() -> CmdResult {
        preprocess()
    }
}