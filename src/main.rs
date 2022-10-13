use clap::{Command, Arg, ArgMatches};
use tokio::net::UdpSocket;
use tracing::{debug, info};
use std::net::SocketAddr;

async fn retranslator(argmatchorig: &ArgMatches) -> Result<(),Box<dyn std::error::Error>> {
    let listen_port: u32 = *argmatchorig.get_one("listen").unwrap();
    let send_ports: Vec<u32> = argmatchorig.get_many::<u32>("ports")
        .unwrap()
        .copied()
        .collect();

    info!("listen port: {}", listen_port);
    info!("send ports: {:?}", send_ports);

    let listen_socket = UdpSocket::bind(format!("0.0.0.0:{}", listen_port)).await?;
    let client_addrs: Vec<SocketAddr> = send_ports
        .iter()
        .map(|portnum| {
            let addr:SocketAddr = format!("127.0.0.1:{}",portnum).parse().unwrap();
            addr
        })
        .collect();

    let mut recvbuf = [0; 4096];
    loop {
        let size = listen_socket.recv(&mut recvbuf).await?;
        debug!("recv {} bytes: {}", size, std::str::from_utf8(&recvbuf[..size])?);

        for dest in &client_addrs {
            listen_socket.send_to(&recvbuf[..size], dest).await?;
            debug!("sent {} bytes to {}", size, dest);
        }

    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cargs = Command::new("simple")
        .version("1.0.2")
        .arg(Arg::new("ports")
            .required(true)
            .short('p')
            .takes_value(true)
            .value_parser(clap::value_parser!(u32))
            .action(clap::ArgAction::Append)
        )
        .arg(Arg::new("listen")
            .required(true)
            .short('l')
            .value_parser(clap::value_parser!(u32))
            .takes_value(true)
        )
        .get_matches();
    retranslator(&cargs).await?;


    Ok(())
}


// fdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfsfdsafdsafdasfdasfs