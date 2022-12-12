use std::{process::{Command, Child, Stdio}, io::{self, BufReader, BufRead}, net::{Ipv4Addr, SocketAddr, SocketAddrV4}, str::FromStr};
use scanf::sscanf;

#[derive(Debug)]
pub enum KubeProxyErr {
    Startup(io::Error),
    ChildRead(Option<io::Error>),
    IpParse(std::net::AddrParseError),
}

#[derive(Debug)]
pub struct KubeProxy {
    process: Child,
    pub listen_addr: SocketAddr
}

impl Drop for KubeProxy {
    fn drop(&mut self) {
        self.process.kill().unwrap();
    }
}

pub fn start_kubectl_proxy(port: u16) -> Result<KubeProxy, KubeProxyErr> {
    let mut cmd = Command::new("kubectl");

    cmd.arg("proxy");
    cmd.arg(format!("--port={}", port));
    cmd.arg("--append-server-path=true");
    cmd.stdout(Stdio::piped());

    let mut process: Child = match cmd.spawn() {
        Ok(child) => child,
        Err(e) => return Err(KubeProxyErr::Startup(e)),
    };

    let mut child_output = BufReader::new(match process.stdout.take() {
        Some(child_stdout) => child_stdout,
        None => return Err(KubeProxyErr::ChildRead(None)),
    });

    let (listen_addr, listen_port) = loop {
        let mut output_buf = String::new();

        let line = child_output.read_line(&mut output_buf);

        match line {
            Ok(_) => {
                let mut addr_str = String::new();
                let mut port: u16 = 0;

                match sscanf!(output_buf.as_str(), "Starting to serve on {string}:{u16}", addr_str, port) {
                    Ok(addr) => addr,
                    Err(_) => continue,
                };

                let addr = match Ipv4Addr::from_str(&addr_str) {
                    Ok(addr) => addr,
                    Err(e) => return Err(KubeProxyErr::IpParse(e)),
                };

                break (addr, port);
            },
            Err(e) => return Err(KubeProxyErr::ChildRead(Some(e))),
        }
    };

    Ok(
        KubeProxy {
            process,
            listen_addr: SocketAddr::V4(
                SocketAddrV4::new(
                    listen_addr,
                    listen_port
                )
            )
        }
    )
}
