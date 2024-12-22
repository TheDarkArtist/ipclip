use std::process::{exit, Command};

fn main() {
    let command_output = Command::new("sh")
        .arg("-c")
        .arg("ip route get 1 | grep -oP 'src \\K\\S+' | xclip -selection clipboard")
        .spawn()
        .expect("Failed to spawn command")
        .wait()
        .expect("Failed to wait for command");

    if command_output.success() {
        println!("Local IP address copied to clipboard successfully.");
    } else {
        eprintln!("Failed to copy IP address to clipboard.");
        exit(1);
    }
}
