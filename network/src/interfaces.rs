use std::pin::Pin;
use libp2p::PeerId;
use std::borrow::Cow;

/// Abstraction over a network.
pub trait Network<B: BlockT> {
    /// Returns a stream of events representing what happens on the network.
    fn event_stream(&self) -> Pin<Box<dyn Stream<Item = Event> + Send>>;

    /// Adjust the reputation of a node.
    fn report_peer(&self, peer_id: PeerId, reputation: ReputationChange);

    /// Force-disconnect a peer.
    fn disconnect_peer(&self, who: PeerId);

    /// Send a notification to a peer.
    fn write_notification(&self, who: PeerId, engine_id: ConsensusEngineId, message: Vec<u8>);

    /// Registers a notifications protocol.
    ///
    /// See the documentation of [`NetworkService:register_notifications_protocol`] for more information.
    fn register_notifications_protocol(
        &self,
        engine_id: ConsensusEngineId,
        protocol_name: Cow<'static, [u8]>,
    );

    /// Notify everyone we're connected to that we have the given block.
    ///
    /// Note: this method isn't strictly related to gossiping and should eventually be moved
    /// somewhere else.
    fn announce(&self, block: B::Hash, associated_data: Vec<u8>);
}