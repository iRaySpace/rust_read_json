use std::fs::File;
use serde_json::Value;

#[derive(Debug)]
struct Server {
    server_name: String,
    server_description: String,
    server_address: String,
    server_port: u16,
    server_utilization: u8,
    server_capacity: u8,
    server_enabled: bool,
}

fn get_servers(server_options: &Vec<Value>) -> Vec<Server> {
    let mut servers = vec![];
    for server_option in server_options {
        let server = Server {
            server_name: String::from(server_option["server_name"].as_str().unwrap()),
            server_description: String::from(server_option["server_description"].as_str().unwrap()),
            server_address: String::from(server_option["server_address"].as_str().unwrap()),
            server_port: server_option["server_port"].as_u64().unwrap() as u16,
            server_utilization: server_option["server_utilization"].as_u64().unwrap() as u8,
            server_capacity: server_option["server_capacity"].as_u64().unwrap() as u8,
            server_enabled: server_option["server_enabled"].as_bool().unwrap(),
        };
        servers.push(server);
    }
    servers
}

fn main() {
    let file = File::open("assets/directory.json").expect("file read-only");
    let json: serde_json::Value = serde_json::from_reader(file).expect("JSON not well-formatted");
    let server_options = json.get("server_options").unwrap().as_array().unwrap();
    let servers = get_servers(server_options);
    for server in servers {
        println!("{:?}", server);
    }
}
