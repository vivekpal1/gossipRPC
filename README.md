# GossipRPC: A Decentralized RPC Implementation using libp2p

## Problem Statement
Traditional RPC systems like gRPC typically rely on a client-server architecture, which can create single points of failure and scalability bottlenecks. In decentralized systems, we need communication patterns that can work without central servers while maintaining the familiar RPC programming model that developers are comfortable with.

## Project Description
GossipRPC demonstrates how to implement RPC-style communication in a fully decentralized manner using libp2p's gossipsub protocol. Instead of direct client-server connections, our implementation creates a peer-to-peer network where:

1. Each node can act as both a service provider and a consumer
2. Messages are propagated through the network using gossip protocols
3. Request-response patterns are maintained despite the lack of direct connections
4. Service discovery happens automatically through the peer-to-peer network

### Core Components

Our implementation consists of four main parts:

1. **Network Behavior** (`behaviour.rs`):
   - Defines how our nodes interact with the libp2p network
   - Implements the gossipsub protocol for message propagation
   - Handles peer discovery and message routing

2. **Node Management** (`node.rs`):
   - Controls the lifecycle of each network node
   - Manages connections and message handling
   - Implements the core networking logic

3. **RPC Protocol** (`rpc.rs`):
   - Defines the structure of RPC messages
   - Implements request-response matching
   - Handles serialization and message formatting

4. **CLI Interface** (`main.rs`):
   - Provides a user-friendly command-line interface
   - Supports running nodes in server or client mode
   - Handles user input and displays results

### How It Works

When you make an RPC call in our system:

1. The caller creates an RPC request with a unique ID
2. The request is published to the gossipsub network on a specific topic
3. All nodes subscribed to that topic receive the request
4. The appropriate node(s) process the request and publish responses
5. Responses are matched to requests using the unique IDs

## Future Extensions

This basic implementation can be extended in several powerful ways:
- Add protobuf-style service definitions
- Code generation for type-safe RPC calls
- Add support for bidirectional streaming
- Message prioritization
- Add Solana blockchain integration for decentralized data streaming
- Implement WebAssembly support for browser-based nodes
- Add support for other transport protocols
- Implement bridges to traditional RPC systems

[Rest of the standard README sections follow...]