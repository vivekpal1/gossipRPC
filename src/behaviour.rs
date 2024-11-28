use libp2p::swarm::NetworkBehaviour;
use libp2p::gossipsub::{
    self,
    Behaviour as GossipsubBehaviour,
    Event as GossipsubEvent,
    IdentTopic,
    MessageAuthenticity,
};

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
#[behaviour(out_event = "BehaviourEvent")]
pub struct RpcBehaviour {
    pub gossipsub: GossipsubBehaviour,
}

#[derive(Debug)]
pub enum BehaviourEvent {
    Gossipsub(GossipsubEvent),
}

impl From<GossipsubEvent> for BehaviourEvent {
    fn from(event: GossipsubEvent) -> Self {
        BehaviourEvent::Gossipsub(event)
    }
}