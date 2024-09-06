use clap::Parser;
use libp2p::{
    autonat, futures::StreamExt, identify, identity, multiaddr::Protocol, swarm::{NetworkBehaviour, SwarmEvent}, Multiaddr, SwarmBuilder
};
use rand::rngs::OsRng;
use std::{error::Error, net::Ipv4Addr, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse();

    let mut swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_quic()
        .with_behaviour(|key| Behaviour::new(key.public()))?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    swarm.listen_on(
        Multiaddr::empty()
            .with(Protocol::Ip4(Ipv4Addr::UNSPECIFIED))
            .with(Protocol::Udp(opt.listen_port))
            .with(Protocol::QuicV1),
    )?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => println!("{event:?}"),
            e => println!("{e:?}"),
        }
    }
}

#[derive(NetworkBehaviour)]
struct Behaviour {
    autonat: autonat::v2::server::Behaviour,
    identify: identify::Behaviour,
}

impl Behaviour {
    pub fn new(key: identity::PublicKey) -> Self {
        Self {
            autonat: autonat::v2::server::Behaviour::new(OsRng),
            identify: identify::Behaviour::new(identify::Config::new("/ipfs/0.1.0".into(), key)),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(name = "libp2p autonatv2 server")]
struct Opt {
    #[clap(short, long, default_value_t = 0)]
    listen_port: u16,
}
