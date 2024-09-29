use colored::Colorize;
use home::home_dir;
use monitors::Profile;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod monitors;

fn main() {
    let target_profile: String = {
        let current_args: Vec<String> = std::env::args().collect();
        current_args.get(1).expect("not enough arguments").clone()
    };

    let config_file_path = match env::var("XDG_CONFIG_HOME") {
        Ok(path) => {
            let mut path = PathBuf::from(path);
            path.push("nocturne");
            path.push("monitors.json");
            path
        }
        Err(_) => {
            let mut path = home_dir().expect("could not find home directory");
            path.push(".config");
            path.push("nocturne");
            path.push("monitors.json");
            path
        }
    };

    let mut file_data: Vec<u8> = vec![];

    let _ = File::open(config_file_path.as_path())
        .expect(
            format!(
                "unable to find or access config file at {}",
                config_file_path.display()
            )
            .as_str(),
        )
        .read_to_end(&mut file_data);

    let profiles: Vec<Profile> =
        serde_json::from_slice(&file_data.as_slice()).expect("failed to parse json");

    for profile in profiles {
        if profile.name == target_profile {
            println!("Configuring Monitors According to Profile {}.", profile.name.bold().green());
	    for hypr_strings in profile.hyprland_strings() {
		println!("Monitor {}", hypr_strings.bold().blue())
	    }
            profile.configure_monitors();
            return;
        } else {
            continue;
        }
    }

    eprintln!("no profile named {} in confiuration", target_profile);
}
