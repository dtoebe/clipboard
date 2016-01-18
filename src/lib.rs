
use std::io::{Read, Write};
use std::process::{exit, Command, Stdio};

#[cfg(target_os = "linux")]
pub fn linux_clipboard(s: &str) {
    match Command::new("which").arg("xclip").status() {
        Ok(status) => {
            if !status.success() {
                println!("You must install xclip from your package manager");
                exit(0);
            }
        }
        Err(_) => {
            println!("Oops something went wrong... Try again");
            exit(0);
        }
    };

    let echo_cmd = Command::new("echo")
        .arg(&s)
        .stdout(Stdio::piped())
        .spawn().unwrap();

    let xclip_cmd = Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn().unwrap();

    if let Some(mut stdout) = echo_cmd.stdout {
        if let Some(mut stdin) = xclip_cmd.stdin {
            let mut buf: Vec<u8> = Vec::new();
            stdout.read_to_end(&mut buf).unwrap();
            stdin.write_all(&buf).unwrap();
        }
    }
    println!("Copied to clipboard..");
}
