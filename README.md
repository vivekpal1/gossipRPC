# GossipRPC

GossipRPC demonstrates how to implement RPC-style communication using libp2p's gossipsub protocol. Instead of direct client-server connections, our implementation spreads messages across a peer-to-peer network, allowing for more resilient and scalable communication patterns.

### Key Features
Our implementation provides several core features that make it a valuable starting point for decentralized applications:

- **Decentralized Communication**: Uses libp2p's gossipsub to enable peer-to-peer message passing without central servers.
- **RPC-Like Interface**: Maintains familiar request-response patterns while leveraging peer-to-peer infrastructure.

### Technical Architecture
The project is organized into four main components:

1. **Network Behavior** (behaviour.rs)
   - Implements libp2p's NetworkBehaviour trait
   - Manages gossipsub message handling
   - Defines custom event types

2. **Node Management** (node.rs)
   - Handles peer-to-peer network operations
   - Manages message routing and processing
   - Implements the gossipsub

3. **RPC Protocol** (rpc.rs)
   - Defines request and response structures
   - Handles message serialization/deserialization
   - Provides a clean interface for RPC operations

4. **CLI Interface** (main.rs)
   - Offers command-line interface for interaction
   - Manages program initialization
   - Handles user commands

### How to Use
1. Install Rust and build the project:
```bash
cargo build --release
```

2. Start a node:
```bash
cargo run -- server --addr "/ip4/127.0.0.1/tcp/8080"
```

3. Make RPC calls:
```bash
cargo run -- call --method "hello" --params '{"name": "world"}'
```

## Future Extensions
1. **Service Definitions**
   - Add protobuf-style service definitions
   - Implement type safety for requests and responses

2. **Messaging Patterns**
   - Implement bi-directional streaming
   - Add support for pub/sub patterns
   - Implement request timeout and retry mechanisms

3. **Network Improvements**
   - Add peer discovery mechanisms
   - Implement connection management
   - Add support for NAT traversal

4. **Solana Integration**
   - Add support for Solana account subscriptions
   - Implement block and transaction streaming
   - Enable decentralized indexing capabilities

## Contributing
Contributions are welcome! Please feel free to submit pull requests. For major changes, please open an issue first to discuss what you would like to change.

## License
This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments
- Built with [libp2p](https://libp2p.io/)