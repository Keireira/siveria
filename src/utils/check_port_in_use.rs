use std::net::TcpStream;
use std::process::Command;
use std::time::Duration;

pub fn check_port_in_use(host: &str, port: &u16) -> (bool, String) {
    /*
     * First try connecting to the port
     */
    match TcpStream::connect_timeout(
        &format!("{}:{}", host, port).parse().unwrap(),
        Duration::from_secs(1),
    ) {
        Ok(_) => {
            /*
             * If we can connect, port is in use, try to get process info
             */
            let mut process_info = String::from("Port is in use");

            if let Ok(output) = Command::new("sh")
                .arg("-c")
                .arg(&format!("lsof -i :{} | tail -n +2", port))
                .output()
            {
                if let Ok(output_str) = String::from_utf8(output.stdout) {
                    process_info = output_str;
                }
            }

            (true, process_info)
        }
        Err(_) => (false, String::from("Port is available")),
    }
}
