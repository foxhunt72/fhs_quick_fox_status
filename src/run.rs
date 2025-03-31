use clap::{Arg, Command as ClapCommand};
use std::process::{Command, Stdio};

fn main() {
    let matches = ClapCommand::new("my_runner")
        .version("0.1.1")
        .author("Richard de Vos")
        .about("Runs a command with arguments while inheriting the terminal environment")
        .arg(
            Arg::new("command")
                .required(true)
                .help("The command to run"),
        )
        .arg(
            Arg::new("args")
                .help("Arguments for the command")
                .num_args(1..)
                .trailing_var_arg(true),
        )
        .arg(
            Arg::new("no-env")
                .long("no-env")
                .help("Disable environment variable inheritance")
                .action(clap::ArgAction::SetTrue),
        )
                .arg(Arg::new("service")
                .required(true)
                .long("service")
                .short('s')
                .help("Service name")
        )
        .arg(Arg::new("valid")
                .required(true)
                .long("valid")
                .short('v')
                .help("Validity duration (e.g., 10m, 1h)")
        )
        .arg(
            Arg::new("description")
                .long("description")
                .short('d')
                .help("Optional description"),
        )
        .arg(
            Arg::new("qfs_command")
                .long("qfs_command")
                .help("qfs_command default: /usr/local/bin/quick_fox_status"),
        )
        .get_matches();

    let command = matches.get_one::<String>("command").unwrap();
    let args: Vec<&str> = matches
        .get_many::<String>("args")
        .unwrap_or_default()
        .map(|s| s.as_str())
        .collect();

    let service = matches.get_one::<String>("service").unwrap();
    let valid = matches.get_one::<String>("valid").unwrap();
    let description_default = "".to_string();
    let description = matches.get_one::<String>("description").unwrap_or(&description_default);
    let qfs_command_default = "/usr/local/bin/quick_fox_status".to_string();
    let qfs_command = matches.get_one::<String>("qfs_command").unwrap_or(&qfs_command_default);

    let mut cmd = Command::new(command);
    cmd.args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    // Inherit environment by default, unless --no-env is specified
    if matches.get_flag("no-env") {
        cmd.env_clear();
    }

    let status = cmd.status().expect("Failed to execute command");
    let status_str: String;

    if status.success()
    {
        println!("Command returned ok.");
        status_str = "ok".to_string()
    }
    else
    {
        println!("Error: Command exited with status: {}", status);
        status_str = "critical".to_string();
    }
    let status_args = vec![
        "add-service".to_string(),
        "--service".to_string(),
        service.to_string(),
        "--description".to_string(),
        description.to_string(),
        "--valid".to_string(),
        valid.to_string(),
        "--status".to_string(),
        status_str.to_string(),
    ];
    // println!("command: {}", status_args.join(" "));
    let output = Command::new(qfs_command)
        .args(&status_args)
        .status();

    if let Ok(status) = output {
        if status.success() {
            println!("status updated");
        } else {
            println!("failed status update");
        }
    } else {
        println!("failed to execute status command");
    }

}

