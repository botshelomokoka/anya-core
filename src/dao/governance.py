"""
This module handles the governance aspects of the Anya DAO, including voting and proposal management.
"""

from anya_core.network import bitcoin_client, rsk_client
from anya_core.utils import helpers
from anya_core.constants import ANYA_TOKEN_CONTRACT_ADDRESS
from .proposal import create_proposal, is_proposal_valid
from .membership_management import is_member
from .executor import execute_proposal

from web3 import Web3

# Initialize Storj client (replace with your actual connection details)
storj_client = StorjClient(access_key='...', secret_key='...', bucket_name='anya-dao')

# Connect to an RSK node
w3 = Web3(Web3.HTTPProvider(rsk_client.RSK_NODE_URL))

# Placeholder for the ANYA token contract ABI (replace with your actual ABI)
ANYA_TOKEN_ABI = ...

# Data storage functions using Storj

def store_proposal(proposal, txid):
    """Stores a new proposal in Storj."""
    proposal_id = proposal['id']
    storj_client.upload_json(f'proposals/{proposal_id}.json', proposal)

def get_proposal_by_id(proposal_id):
    """Retrieves a proposal by its ID from Storj."""
    try:
        return storj_client.download_json(f'proposals/{proposal_id}.json')
    except Exception as e: 
        print(f"Error fetching proposal {proposal_id}: {e}")
        return None

def get_votes_for_proposal(proposal_id):
    """Retrieves all votes for a specific proposal from Storj."""
    try:
        return storj_client.download_json(f'votes/{proposal_id}.json') or []
    except Exception as e:
        print(f"Error fetching votes for proposal {proposal_id}: {e}")
        return []

def record_vote(proposal_id, voter_address, vote_option, amount, txid):
    """Records a vote in Storj."""
    vote = {
        'voter_address': voter_address,
        'option': vote_option,
        'amount': amount,
        'txid': txid
    }
    votes = get_votes_for_proposal(proposal_id)
    votes.append(vote)
    storj_client.upload_json(f'votes/{proposal_id}.json', votes)

def update_proposal_status(proposal_id, new_status):
    """Updates the status of a proposal in Storj."""
    proposal = get_proposal_by_id(proposal_id)
    if proposal:
        proposal['status'] = new_status
        storj_client.upload_json(f'proposals/{proposal_id}.json', proposal)

def get_current_epoch():
    """
    Retrieves the current epoch from Storj
    """

    try:
        return storj_client.download_json('epoch.json')['current_epoch']
    except Exception as e:
        print(f"Error fetching current epoch: {e}")
        return 0 

def set_current_epoch(new_epoch):
    """
    Sets the current epoch in Storj
    """

    storj_client.upload_json('epoch.json', {'current_epoch': new_epoch})


# Main governance functions

def submit_proposal(proposer, title, description, options, start_time=None, end_time=None, chain='bitcoin'):
    """
    Submits a new proposal to the DAO.
    """

    # 1. Validate proposal data
    if not is_member(proposer):
        raise ValueError("Only DAO members can submit proposals")

    proposal = create_proposal(proposer, title, description, options, start_time, end_time, chain)
    
    if not is_proposal_valid(proposal):
        raise ValueError("Invalid proposal")

    # 2. Encode proposal data into an OP_RETURN transaction output
    op_return_data = helpers.encode_proposal_data(proposal)

    # 3. Construct and broadcast the transaction using bitcoin_client
    tx = bitcoin_client.create_op_return_transaction(op_return_data, proposal_data['proposer']) 
    txid = bitcoin_client.broadcast_transaction(tx)

    if txid:
        # 4. Store the proposal 
        store_proposal(proposal, txid)

    return txid

def get_proposals():
    """
    Retrieves a list of active proposals.
    """

    # Fetch proposals from Bitcoin 
    bitcoin_proposals = _get_proposals_from_bitcoin()

    # Fetch proposals from RSK (if applicable)
    rsk_proposals = [] 
    if ANYA_TOKEN_CONTRACT_ADDRESS:
        rsk_proposals = _get_proposals_from_rsk() 

    # Combine and filter active proposals
    all_proposals = bitcoin_proposals + rsk_proposals
    active_proposals = [p for p in all_proposals if p['status'] == 'active']

    return active_proposals

def _get_proposals_from_bitcoin():
    """
    Retrieves proposals from the Bitcoin blockchain
    """

    proposal_transactions = bitcoin_client.get_op_return_transactions()

    proposals = []
    for tx in proposal_transactions:
        try:
            proposal_data = helpers.decode_proposal_data(tx['op_return'])
            if is_proposal_valid(proposal_data):
                proposals.append(proposal_data)
        except Exception as e: 
            print(f"Error decoding proposal data from transaction {tx['txid']}: {e}")

    return proposals

def _get_proposals_from_rsk():
    """
    Retrieves proposals from the RSK network
    """

    # 1. Use rsk_client to interact with the DAO contract and fetch active proposals
    dao_contract = w3.eth.contract(address=ANYA_TOKEN_CONTRACT_ADDRESS, abi=ANYA_TOKEN_ABI)
    
    # Get the total number of proposals
    proposal_count = dao_contract.functions.proposalCount().call()

    # Iterate through proposals and filter for active ones
    active_proposals = []
    for proposal_id in range(1, proposal_count + 1):
        (
            proposer,
            targets, 
            values, 
            calldatas, 
            description, 
            startBlock, 
            endBlock,
            proposalType, 
            ipfsHash
        ) = dao_contract.functions.proposals(proposal_id).call()

        # Check if the proposal is active
        if block.number >= startBlock && block.number <= endBlock:
            # You might need to fetch additional details from IPFS using the `ipfsHash`
            proposal = {
                'id': proposal_id,
                'proposer': proposer,
                'targets': targets,
                'values': values,
                'calldatas': calldatas,
                'description': description,
                'start_time': startBlock,
                'end_time': endBlock,
                'proposal_type': proposalType, 
                'chain': 'rsk'
            }
            active_proposals.append(proposal)

    return active_proposals


def vote_on_proposal(proposal_id, vote_option, voter_address, amount):
    """
    Casts a vote on a specific proposal.
    """

    if not is_member(voter_address):
        raise ValueError("Only DAO members can vote")

    proposal = get_proposal_by_id(proposal_id) 
    if not proposal or proposal['status'] != 'active':
        raise ValueError("Invalid or inactive proposal")

    if vote_option not in proposal['options']:
        raise ValueError("Invalid vote option")

    # Cast vote on the appropriate chain
    if proposal['chain'] == 'bitcoin':
        vote_data = helpers.encode_vote_data(proposal_id, vote_option, amount)
        tx = bitcoin_client.create_op_return
