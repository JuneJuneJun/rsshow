
use libp2p::{Transport, core::upgrade, tcp::TcpConfig, secio::SecioConfig, identity::Keypair, yamux};

pub fn test_libp2p() {
    let tcp = TcpConfig::new();
    let secio = SecioConfig::new(Keypair::generate_ed25519());
}