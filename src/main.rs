
const HOSTNAME_FILE: &str = "/proc/sys/kernel/hostname";
const OS_RELEASE_FILE: &str = "/etc/os-release";

fn main() {
    let hostname = rustyfetch::get_command_results("cat", HOSTNAME_FILE);
    let distro = rustyfetch::get_distro_name(OS_RELEASE_FILE).expect("");
    let shell = rustyfetch::get_shell();
    let uptime = rustyfetch::get_command_results("uptime", "-p").replace("up","");

    println!("{} @ {}", rustyfetch::get_var("USER"), &hostname);
    println!("{}: {}", "distro", distro);
    println!("{}: {}", "terminal", rustyfetch::get_var("TERM"));
    println!("{}: {}", "editor", rustyfetch::get_var("EDITOR"));
    println!("{}: {}", "shell", shell);
    println!("{}: {}", "uptime", uptime.trim());
    rustyfetch::loop_colors();

    // TODO: 
    // Lets rethink how to add padding to each side and underline the 
    // User @ Hostname portion.
    // This isnt very modular at the moment...
    // I will eventually need a function that executes the print
    // statements.. Maybe a struct that holds the key value pairs
    // and a sepearte part for the heading. I want the user to have
    // a config file at ~/.config/rustyfetch/config.toml
    // I also should probably (?) use clap as well
    // The appeal is it will be very customizable so I need to think of 
    // how I am going to have ways to turn modules ON/OFF and 
    // have the user be able to customize the output
}
