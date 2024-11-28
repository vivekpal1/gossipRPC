use anyhow::Result;
use futures::StreamExt;
use libp2p::{
    core::{transport::upgrade, Transport},
    gossipsub::{self, IdentTopic, MessageAuthenticity},
    identity, noise,
    swarm::SwarmEvent,
    tcp, yamux, Multiaddr, PeerId, Swarm,
};
use std::collections::HashMap;
use tokio::sync::mpsc;

use crate::{
    behaviour::{BehaviourEvent, RpcBehaviour},
    rpc::{RpcRequest, RpcResponse},
};

pub struct Node {
    swarm: Swarm<RpcBehaviour>,
    topic: IdentTopic,
    pending_requests: HashMap<String, mpsc::Sender<RpcResponse>>,
}

impl Node {
    pub async fn new() -> Result<Self> {
        let id_keys = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(id_keys.public());

        let transport = tcp::tokio::Transport::new(tcp::Config::default())
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::Config::new(&id_keys)?)
            .multiplex(yamux::Config::default())
            .boxed();

        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .build()
            .expect("Valid gossipsub config");
        
        let gossipsub = gossipsub::Behaviour::new(
            MessageAuthenticity::Signed(id_keys),
            gossipsub_config,
        ).expect("Valid gossipsub behavior");
        let behaviour = RpcBehaviour { gossipsub };
        let swarm = Swarm::new(
            transport,
            behaviour,
            peer_id,
            libp2p::swarm::Config::with_tokio_executor(),
        );

        Ok(Self {
            swarm,
            topic: IdentTopic::new("gossiprpc"),
            pending_requests: HashMap::new(),
        })
    }

    pub async fn start(&mut self, addr: Multiaddr) -> Result<()> {
        self.swarm.listen_on(addr)?;
        self.swarm.behaviour_mut().gossipsub.subscribe(&self.topic)
            .expect("Valid topic subscription");

        while let Some(event) = self.swarm.next().await {
            match event {
                SwarmEvent::Behaviour(BehaviourEvent::Gossipsub(
                    gossipsub::Event::Message { message, .. }
                )) => {
                    self.handle_message(message).await?;
                }
                _ => {} 
            }
        }

        Ok(())
    }

    async fn handle_message(&mut self, message: gossipsub::Message) -> Result<()> {
        let content = String::from_utf8(message.data)?;
        
        if let Ok(request) = serde_json::from_str::<RpcRequest>(&content) {
            let response = RpcResponse::success(
                request.id.clone(),
                serde_json::json!("Method executed successfully"),
            );
            
            let response_data = serde_json::to_string(&response)?;
            self.swarm.behaviour_mut().gossipsub.publish(
                self.topic.clone(),
                response_data.as_bytes(),
            )?;
        }

        Ok(())
    }
}