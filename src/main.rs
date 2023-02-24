use std::env;
use std::fs;
use std::process::{Command, Stdio};
use std::str;
// use regex::RegexSet;

const LINE_LENGTH: u32 = 30;
const PAD_TOP: u32 = 2;
const PAD_BOTTOM: u32 = 2;
const HOSTNAME_FILE: &str = "/proc/sys/kernel/hostname";
const OS_RELEASE_FILE: &str = "/etc/os-release";
const DASH = String::from("-");
    

let nl = String::from("\n");

fn get_var(envvar: &str) -> String {
    let return_val = env::var(envvar).expect("");
    return return_val;
}

fn get_command_results(command: &str, command_arg: &str) -> String {
    let command_res = Command::new(command)
        .arg(command_arg)
        .stdout(Stdio::piped())
        .spawn()
        .expect("");
    let output = command_res.wait_with_output().expect("");
    assert!(output.status.success());
    let finalres = str::from_utf8(&output.stdout).expect("");
    return finalres.to_string();
}

fn print_repeat(printed: &str, n: u32) {
    // print a line
    let mut linecount = 0;
    print!("\t");
    while linecount < n {
        print!("{}", printed);
        linecount = linecount + 1;
    }
    print!("\n");
}

fn loop_colors() {
    let mut i = 30;
    let symbol = String::from("⬤ ");
    print!("\t");
    while i < 38 {
        let j = i + 60;
        print!("\x1b[1;{}m{}", i, symbol);
        print!("\x1b[1;{}m{}", j, symbol);
        i = i + 1
    }
    println!()
}

fn main() {
    // get hostname with bash command
    let hostname = get_command_results("cat", HOSTNAME_FILE).replace("\n", "");

    // get distro from os release file to a string
    let contents_release = fs::read_to_string(OS_RELEASE_FILE).expect("");
    let contents_release_clean = contents_release.replace("\"", "");
    // why does split require single quotes?
    let vecs_release: Vec<&str> = contents_release_clean.split(&['\n', '=']).collect();
    let index_distro = vecs_release.iter().position(|&r| r == "NAME").unwrap();
    let sel_distro = index_distro + 1; // find the string index of Vec after where value is "NAME"
    let distro = vecs_release[sel_distro];

    // get shell and clean to just name /usr/share/zsh -> zsh
    let shell_response = get_var("SHELL");
    let shell_vecs: Vec<&str> = shell_response.split("/").collect();
    let shell_sel = shell_vecs.len();
    let shell = shell_vecs[shell_sel - 1];

    let uptime_response = get_command_results("uptime", "-p");
    let uptime = uptime_response.replace("up", "");

    // everything formatted and output to terminal

    print_repeat(&nl, PAD_TOP);
    println!("\t{} @ {}", get_var("USER"), &hostname);
    print_repeat(&dash, LINE_LENGTH);
    println!("\t{}: {}", "distro", distro);
    println!("\t{}: {}", "terminal", get_var("TERM"));
    println!("\t{}: {}", "editor", get_var("EDITOR"));
    println!("\t{}: {}", "shell", shell);
    println!("\t{}: {}", "uptime", uptime.trim());
    loop_colors();
    print_repeat(&nl, PAD_BOTTOM);

    // TODO: get cpu specs
    // cat /proc/cpuinfo
    // let contents_cpu = fs::read_to_string("/proc/cpuinfo").expect("None");
    // let set = RegexSet::new([r":",r"/t"]).unwrap();
    // Iterate over and collect all of the matches.
    // let matches: Vec<_> = set.matches(&contents_cpu).into_iter().collect();
    // println!("{:?}", matches);
    // let contents_cpu_clean = contents_cpu.replace("\t","");
    // let vecs_cpu: Vec<&str> = contents_cpu.split(&[':']).collect();
    //
    // println!("{:?}", vecs_cpu);
    // let index_modelname = vecs_cpu.iter().position(|&r| r == "model name").unwrap();
    // let sel_modelname = index_modelname + 1;
    // let modelname = vecs_cpu[sel_modelname];
    // println!("\t{}: {}", "cpu model", modelname);

    // clean shell response to get final string in path

    // do color loop

    // done
}
