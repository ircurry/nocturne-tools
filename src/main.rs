use std::io::{self, Write};

mod monitors;

fn main() {
    let profile = monitors::Profile::new_profile1();
    println!("Configuring Monitors for Profile {}", profile.name);
    let profile_output = profile.configure_monitors();
    for output in profile_output {
	io::stdout().write_all(&output.stdout).expect("failed to write to stdout");
	io::stderr().write_all(&output.stderr).expect("failed to write to stderr");
    }
}
