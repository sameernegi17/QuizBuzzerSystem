use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/typescript/");
    println!("cargo:rerun-if-changed=build.rs");

    // Create a new Command instance for the subcommand you want to execute
    let mut subcommand = Command::new("tsc");

    // Set the current working directory for the subcommand
    subcommand.current_dir("src/typescript");

    // Execute the subcommand
    let output = subcommand.output().expect("Failed to execute command");

    // Check the output of the subcommand
    if output.status.success() {
        // Subcommand executed successfully
        println!("TypeScript compilation succeeded");
    } else {
        // Subcommand failed
        let error_message = String::from_utf8_lossy(&output.stderr);
        println!(
            "TypeScript compilation failed with error: {}",
            error_message
        );
    }
}
