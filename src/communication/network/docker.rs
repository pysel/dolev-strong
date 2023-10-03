use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct DockerSocket {
    pub hostname: String,
    pub port: u16,
}

impl DockerSocket {
    pub fn tuple(&self) -> (String, u16) {
        (self.hostname.clone(), self.port)
    }

    pub fn from_string(string: &str) -> DockerSocket {
        let splitted: Vec<&str> = string.split(":").collect();
        DockerSocket { hostname: splitted[0].to_string(), port: splitted[1].parse::<u16>().unwrap() }
    }
}

// wait_until_containers_are_up waits until all containers are up before continuing execution.
// this is done by checking if a file /tmp/start.txt exists (which is created after docker compose launches all containers)
pub fn wait_until_containers_are_up() {
    while !Path::new("/tmp/start.txt").exists() {}
}