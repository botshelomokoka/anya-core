# Network Types

The Anya Core Network is a decentralized network of nodes that can run different types of networks. Currently, the following network types are supported:

- **Bitcoin**: Connect to the Bitcoin network to run a full node, SPV node, or a pruned node.
- **Lightning**: Connect to the Lightning Network to create and manage Lightning channels.
- **Stacks**: Connect to the Stacks blockchain network to run a full node, SPV node, or a pruned node.
- **IPFS**: Connect to the InterPlanetary File System (IPFS) to store and retrieve files.
- **Unified**: Connect to the Unified Network to run a full node, SPV node, or a pruned node. The Unified Network is a custom network that can be used to run any type of network.

The type of network that a node can connect to is determined by the `network` field in the node's configuration file. The `network` field can be set to one of the following values:

- `bitcoin`
- `lightning`
- `stacks`
- `ipfs`
- `unified`

By default, the `network` field is set to `bitcoin`.

The type of network that a node can connect to determines the type of network messages that the node can send and receive. It also determines the type of network peers that the node can connect to.

*Last updated: 2024-12-07*
