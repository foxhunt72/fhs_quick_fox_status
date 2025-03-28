use clap::{Arg, ArgMatches, Command}; // Import ArgMatches
mod myconfig;
mod tn;

fn sanitize_name(input: &str) -> String {
    input.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' => c,
            _ => '_', // Replace unsafe characters
        })
        .collect()
}

fn main() {
    let config = match myconfig::Settings::load() {
        Ok(cfg) => {
            // println!("Loaded config: {:?}", cfg);
            cfg
        },
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            // You have a few options here:
            // 1. Use a default config
            myconfig::Settings::default()
            // 2. Or exit the program
            // std::process::exit(1);
        },
    };

    let matches = Command::new("quick fox status CLI")
        .version("0.1.0")
        .author("Richard de Vos")
        .about("CLI for managing Quick Fox Status")
        .subcommand(Command::new("discovery").about("list discovery for zabbix"))
        .subcommand(
            Command::new("discovery-check")
                .about("Check a discovered name")
                .arg(Arg::new("name")
                         .long("name")
                         .short('n')
                         .required(true)
                         .help("name to check, see discovery")
                ),
        )
        .subcommand(
            Command::new("list-services")
                .about("List all services")
                .arg(
                    Arg::new("view")
                        .short('v')
                        .long("view")
                        .value_parser(["json", "text"])
                        .default_value("text")
                        .help("[not-implemented yet] View format"),
                )
                .arg(
                    Arg::new("filter")
                        .short('f')
                        .long("filter")
                        .help("[not-implemented yet] Filter by status (comma-separated)"),
                ),
        )
        .subcommand(
            Command::new("status")
                .about("Get service status")
                .arg(Arg::new("name")
                         .long("name")
                         .short('n')
                         .required(true)
                         .help("name to check, see discovery")
                ),
        )
        .subcommand(
            Command::new("add-service")
                .about("Add or update a service")
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
                    Arg::new("status")
                        .required(true)
                        .long("status")
                        .short('t')
                        .value_parser(["ok", "failed", "critical", "warning", "unknown"])
                        .help("Service status")
                )
                .arg(Arg::new("host")
                        .long("host")
                        .short('o')
                        .help("Host name")
                )
                .arg(
                    Arg::new("description")
                        .long("description")
                        .short('d')
                        .help("Optional description"),
                )
                .arg(Arg::new("log")
                        .long("log")
                        .short('l')
                        .help("Service status")
                ),
        )
        .subcommand(
            Command::new("delete-service")
                .about("Delete a service")
                .arg(Arg::new("name")
                         .long("name")
                         .short('n')
                         .required(true)
                         .help("name to check, see list-services")
                ),
        )
        .get_matches();

    handle_commands(&matches, &config);
}

fn handle_commands(matches: &ArgMatches, config: &myconfig::Settings) {
    let qfs_path = &config.config.qfs_path;
    let host_default = &config.config.host;

    match matches.subcommand() {
        Some(("list-services", sub_m)) => {
            println!("Listing all services...");
            let view = sub_m.get_one::<String>("view").unwrap();
            let filter_default = "".to_string();
            let filter = sub_m.get_one::<String>("filter").unwrap_or(&filter_default);
            tn::list_services(qfs_path, host_default, view, filter);
        }
        Some(("discovery", _)) => {
            tn::list_discovery(qfs_path, host_default);
        }
        Some(("discovery-check", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            tn::discovery_check(qfs_path, name);
        }
        Some(("status", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            tn::discovery_check(qfs_path, name);
        }
        Some(("add-service", sub_m)) => {
            // let host_default = "localhost".to_string();
            let host = sanitize_name(sub_m.get_one::<String>("host").unwrap_or(&host_default));
            let service = sanitize_name(sub_m.get_one::<String>("service").unwrap());
            let valid = sub_m.get_one::<String>("valid").unwrap();
            let status = sub_m.get_one::<String>("status").unwrap();

            let description_default = "".to_string();
            let description = sub_m.get_one::<String>("description").unwrap_or(&description_default);

            let log_default = "".to_string();
            let log = sub_m.get_one::<String>("log").unwrap_or(&log_default);
            tn::save_add_server(qfs_path, host_default, service.as_str(), host.as_str(), status, valid, description, log);

            println!(
                "Adding service {} on host {} with status {} for {}. Description: {}. log: {}",
                service, host, status, valid, description, log
            );
        }
        Some(("delete-service", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            println!("Not implemented yet: Deleting service with name: {}", name);
        }
        _ => {
            println!("Unknown command. Use --help for available commands.");
        }
    }
}

