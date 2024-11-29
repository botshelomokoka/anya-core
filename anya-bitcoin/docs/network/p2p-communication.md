# P2P Communication

The AnYa network is a peer-to-peer network where nodes communicate with each other directly. This section provides an overview of the communication protocol and the messages used for communication.

## Connection Establishment

To establish a connection with a peer, a node sends a `connect` message. The `connect` message includes the peer's network address and the node's own network address. If the peer is available to accept connections, it sends a `connection_accepted` message back to the node with its own network address.

## Message Format

Each message consists of a header and a payload. The header contains the message type and the length of the payload in bytes. The payload is the actual data being sent.

The message type is a string of up to 32 bytes. The length of the payload is a 32-bit unsigned integer.

## Message Types

The following message types are supported:

- `connect`: Establish a connection with a peer.
- `connection_accepted`: Confirm that a connection has been established.
- `disconnect`: Request to disconnect from a peer.
- `disconnect_ack`: Acknowledge a request to disconnect.
- `ping`: Send a ping message to a peer.
- `pong`: Respond to a ping message.
- `tx`: Send a transaction to a peer.
- `block`: Send a block to a peer.
- `get_block`: Request a block from a peer.
- `get_blockchain_info`: Request blockchain information from a peer.
- `blockchain_info`: Send blockchain information to a peer.
- `get_mempool`: Request the mempool from a peer.
- `mempool`: Send the mempool to a peer.
- `get_peers`: Request the list of peers from a peer.
- `peers`: Send the list of peers to a peer.

## Message Payloads

The payloads for each message type are as follows:

- `connect`: `network_address` (32 bytes)
- `connection_accepted`: `network_address` (32 bytes)
- `disconnect`: None
- `disconnect_ack`: None
- `ping`: None
- `pong`: None
- `tx`: `transaction` (variable length)
- `block`: `block` (variable length)
- `get_block`: `block_hash` (32 bytes)
- `get_blockchain_info`: None
- `blockchain_info`: `blockchain_info` (variable length)
- `get_mempool`: None
- `mempool`: `mempool` (variable length)
- `get_peers`: None
- `peers`: `peers` (variable length)

## Errors

If an error occurs while processing a message, an error message is sent back to the sender. The error message includes the message type and the error code.

The following error codes are supported:

- `invalid_message`: The message is invalid.
- `invalid_payload`: The payload is invalid.
- `unknown_message_type`: The message type is unknown.
- `unsupported_message_type`: The message type is not supported.
- `invalid_network_address`: The network address is invalid.
