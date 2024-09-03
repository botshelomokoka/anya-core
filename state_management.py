import json
import schnorr
import logging

class Node:
    def __init__(self):
        self.dao_progress = 0
        self.network_state = {}
        self.user_data = {}
        self.federated_nodes = []
        self.schnorr_keypair = schnorr.generate_keypair()
        logging.basicConfig(level=logging.INFO)

    def merge_state(self, remote_state, remote_node_pubkey):
        """
        Merge the remote state into the local state after verifying the signature.
        """
        signature = remote_state.get('signature')
        if not self.verify_signature(signature, remote_state, remote_node_pubkey):
            raise ValueError("Invalid signature")

        for key, value in remote_state.items():
            if key not in self.__dict__:
                continue

            if isinstance(value, dict):
                self.merge_state(getattr(self, key), value)
            else:
                setattr(self, key, value)

    def verify_signature(self, signature, data, pubkey):
        """
        Verify the signature of the given data using the provided public key.
        """
        serialized_data = json.dumps(data, sort_keys=True).encode()
        return schnorr.verify(signature, serialized_data, pubkey)

    def get_state(self):
        """
        Get the current state of the node, excluding sensitive information.
        """
        state = {}
        for key, value in self.__dict__.items():
            if key not in ['federated_nodes', 'schnorr_keypair']:
                state[key] = value
        return state

    def sign_state(self):
        """
        Sign the current state of the node.
        """
        serialized_state = json.dumps(self.get_state(), sort_keys=True).encode()
        signature = schnorr.sign(serialized_state, self.schnorr_keypair.private_key)
        return signature
