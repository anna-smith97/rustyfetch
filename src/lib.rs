use ::std::process::{Command, Stdio};
use ::std::str;
use std::env;
use std::fs;

pub fn get_var(envvar: &str) -> String {
    let return_val = env::var(envvar).expect("");
    return return_val;
}

pub fn get_command_results(command: &str, command_arg: &str) -> String {
    let command_res = Command::new(command)
        .arg(command_arg)
        .stdout(Stdio::piped())
        .spawn()
        .expect("");
    let output = command_res.wait_with_output().expect("");
    assert!(output.status.success());
    let finalres = str::from_utf8(&output.stdout).expect("");
    return finalres.to_string().replace("\n", "");
}


pub fn loop_colors() {
    let mut i = 30;
    let symbol = String::from("⬤ ");
    while i < 38 {
        let j = i + 60;
        print!("\x1b[1;{}m{}", i, symbol);
        print!("\x1b[1;{}m{}", j, symbol);
        i = i + 1
    }
    println!()
}

pub fn get_distro_name(file_path: &str) -> Option<String> {
    let contents_release = fs::read_to_string(file_path).expect("");
    let contents_release_clean = contents_release.replace("\"", "");
    let vecs_release: Vec<&str> = contents_release_clean.split(&['\n', '=']).collect();
    let index_distro = vecs_release.iter().position(|&r| r == "NAME").unwrap();
    let sel_distro = index_distro + 1;
    let distro = vecs_release[sel_distro];
    Some(distro.to_string())
}

pub fn get_shell() -> String {
    let shell_response = get_var("SHELL");
    let shell_vecs: Vec<&str> = shell_response.split("/").collect();
    let shell_sel = shell_vecs.len();
    let shell = shell_vecs[shell_sel - 1];
    shell.to_string()
}
