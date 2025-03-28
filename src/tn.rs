use serde::{Serialize, Deserialize};
mod json;
use glob::glob;
use chrono::{DateTime, Utc, TimeZone};
use prettytable::{Table, Row, Cell};
// use duration2int::parse_duration;
mod duration2int;


#[derive(Serialize, Deserialize, Debug, Clone)]
struct ServiceStatus {
    host: String,
    service: String,
    start: i64,  // Epoch time in seconds
    end: i64,    // Epoch time in seconds
    status: String,
    description: String,
    log: String,
}

impl ServiceStatus {
    fn format_timestamp(epoch: i64) -> String {
        let datetime: DateTime<Utc> = Utc.timestamp_opt(epoch, 0).unwrap();
        datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }
    fn check_status(epoch: i64, status: &str) -> String {
        let current_time = Utc::now().timestamp();
        if current_time > epoch
        {
            "unknown".to_string()
        }
        else
        {
            status.to_string()
        }
    }
}

pub fn save_add_server(qfs_path: &str, host_default: &str, service: &str, host: &str, status: &str, valid: &str, description: &str, log: &str) {
    let service_start = Utc::now().timestamp();
    let duration = duration2int::parse_duration(valid);
    let service_end = service_start + duration;
    let save_file: String;
    if host_default == host
    {
        save_file = format!("{}/{}.json", qfs_path, service);
    }
    else
    {
        save_file = format!("{}/{}_{}.json", qfs_path, host, service);
    }
    let my_data = ServiceStatus { host: host.to_string(), service: service.to_string(), description: description.to_string(), start: service_start, end: service_end, status: status.to_string(), log: log.to_string()};
    // let save_file = format!{"{
    // let _ = json::save_to_json(&my_data, Path::new("data.json"));
    let _ = json::save_to_json(&my_data, save_file.as_ref());

}

pub fn list_services(qfs_path: &str, host_default: &str, _view: &str, _filter: &str) {
    let array_my_data = get_services(qfs_path);

    let mut table = Table::new();
    // println!("vector: {:?}", array_my_data);

    //Add table headers
    table.set_titles(Row::new(vec![
            Cell::new("Name"),
            Cell::new("Host"),
            Cell::new("Service"),
            Cell::new("Start Time"),
            Cell::new("End Time"),
            Cell::new("Status"),
    ]));
    for service in array_my_data {
        let service_name: String;

        if host_default == service.host
        {
            service_name = format!("{}", service.service);
        }
        else
        {
            service_name = format!("{}_{}", service.host, service.service);
        }
        table.add_row(Row::new(vec![
                Cell::new(&service_name),
                Cell::new(&service.host),
                Cell::new(&service.service),
                Cell::new(&ServiceStatus::format_timestamp(service.start)),
                Cell::new(&ServiceStatus::format_timestamp(service.end)),
                Cell::new(&ServiceStatus::check_status(service.end,service.status.as_str())),
        ]));
    }
    table.printstd();
}

pub fn list_discovery(qfs_path: &str, host_default: &str) {
    let array_my_data = get_services(qfs_path);

    println!("[");
    for (i, service) in array_my_data.iter().enumerate() {
        let service_name: String;

        if host_default == service.host
        {
            service_name = format!("{}", service.service);
        }
        else
        {
            service_name = format!("{}_{}", service.host, service.service);
        }
        println!("  {{");
        println!("    \"{{#NAME}}\": \"{}\"", service_name);
        if i == array_my_data.len() - 1 {
            println!("  }}"); // No trailing comma on the last item
        } else {
            println!("  }},");
        }
    }
    println!("]");
}

fn get_services(qfs_path: &str) -> Vec<ServiceStatus> {
    let mut array_my_data: Vec<ServiceStatus> = Vec::new();
    let pattern = format!("{}/*.json", qfs_path);
    for e in glob(&pattern).expect("Failed to read glob pattern") {
        // println!("{}", e.unwrap().display());

        let my_path = e.unwrap();
        // println!("file: {}", my_path.display());
        let _loaded_data = match json::load_from_json::<ServiceStatus>(my_path.as_ref()) {

            Ok(data) => {
                // println!("file: '{}': {} {}", my_path.display(), data.service, data.status);
                array_my_data.push(data);
            },
            Err(e) => {
                println!("Could not load from {}: {}", my_path.display(), e);
            }
        };
    };
    array_my_data
}

pub fn discovery_check(qfs_path: &str, name: &str) {
    let load_file = format!("{}/{}.json", qfs_path, name);
    let _loaded_data = match json::load_from_json::<ServiceStatus>(load_file.as_ref()) {
        Ok(data) => {
            let status = &ServiceStatus::check_status(data.end,data.status.as_str());
            println!("{}: {} {} {}", status, data.host, data.service, data.description);
        },
        Err(_) => {
            println!("notfound: {}", name);
        }
    };
}
