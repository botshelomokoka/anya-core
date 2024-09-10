import asyncio
from libp2p import (
    new_node,
    PeerID,
    multiaddr,
)
from libp2p.crypto.keys import KeyPair
from libp2p.network.swarm import Swarm
from libp2p.security.secio import SecioTransport
from libp2p.stream_muxer.mplex import MPLEXMuxer
from libp2p.transport.tcp import TCP

async def discover_network():
    # Create a random PeerID
    key_pair = KeyPair.generate('ed25519')
    peer_id = PeerID.from_public_key(key_pair.public_key)
    print(f"Local peer id: {peer_id}")

    # Create a new libp2p node
    node = await new_node(
        transport_opt=[TCP()],
        muxer_opt=[MPLEXMuxer()],
        sec_opt=[SecioTransport(key_pair)],
        peer_id=peer_id,
    )

    # Listen on all interfaces and whatever port the OS assigns
    await node.get_network().listen(multiaddr.Multiaddr("/ip4/0.0.0.0/tcp/0"))

    print(f"Node listening on {node.get_addrs()}")

    # Kick it off
    while True:
        await asyncio.sleep(1)  # Add a small delay to prevent busy-waiting

if __name__ == "__main__":
    asyncio.run(discover_network())
