use std::path::Path;
use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut target_dir = "~".to_string();
    if args.len() > 2 {
        println!("Error: more then 2 arguements.");
    } else if args.len() == 2 {
        // for arg in &args[1..] {
        //     println!("  {}", arg);
        // }
        if Path::new(&args[1]).exists() {
            if Path::new(&args[1]).is_absolute() {
                target_dir = args[1].clone();
            } else {
                let current_dir = env::current_dir().expect("Failed to get current directory");
                target_dir = current_dir.join(Path::new(&args[1])).to_string_lossy().into_owned();
            }
        }
    } else {
        // println!("No arguments provided");
        // Get the current directory
        let current_dir = env::current_dir().expect("Failed to get current directory");
        target_dir = current_dir.to_string_lossy().to_string();
    }


    // Run explorer.exe in the current directory
    // let mut child = Command::new("explorer.exe")
    //                   .current_dir(env::current_dir())
    Command::new("explorer.exe")
        .arg(target_dir.replace("/","\\"))
        .spawn()
        .expect("Failed to spawn explorer.exe");

    // Wait for the process to finish
    // let exit_status = child.wait().expect("Failed to wait for explorer.exe");

    // println!("explorer.exe exited with status: {}", exit_status);
}

