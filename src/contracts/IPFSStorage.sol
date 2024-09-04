// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@chainlink/contracts/src/v0.8/ChainlinkClient.sol";

/**
 * @title IPFSStorage
 * @author Botshelo Mokoka
 * @dev A contract to store and retrieve data from IPFS using Chainlink oracles
 */
contract IPFSStorage is Ownable, ChainlinkClient {
    using Chainlink for Chainlink.Request;

    string public ipfsGatewayUrl;
    address private oracle;
    bytes32 private jobId;
    uint256 private fee;

    mapping(bytes32 => string) private requestToCID;
    mapping(bytes32 => bytes) private requestToData;

    event DataStored(bytes32 indexed requestId, string cid);
    event DataRetrieved(bytes32 indexed requestId, bytes data);

    constructor(string memory _ipfsGatewayUrl, address _oracle, bytes32 _jobId, uint256 _fee, address _link) {
        ipfsGatewayUrl = _ipfsGatewayUrl;
        oracle = _oracle;
        jobId = _jobId;
        fee = _fee;
        setChainlinkToken(_link);
    }

    function storeOnIPFS(bytes memory data) public returns (bytes32) {
        Chainlink.Request memory request = buildChainlinkRequest(jobId, address(this), this.fulfillStore.selector);
        request.add("data", string(data));
        bytes32 requestId = sendChainlinkRequestTo(oracle, request, fee);
        return requestId;
    }

    function fulfillStore(bytes32 _requestId, string memory _cid) public recordChainlinkFulfillment(_requestId) {
        requestToCID[_requestId] = _cid;
        emit DataStored(_requestId, _cid);
    }

    function retrieveFromIPFS(string memory cid) public returns (bytes32) {
        Chainlink.Request memory request = buildChainlinkRequest(jobId, address(this), this.fulfillRetrieve.selector);
        request.add("cid", cid);
        request.add("gatewayUrl", ipfsGatewayUrl);
        bytes32 requestId = sendChainlinkRequestTo(oracle, request, fee);
        return requestId;
    }

    function fulfillRetrieve(bytes32 _requestId, bytes memory _data) public recordChainlinkFulfillment(_requestId) {
        requestToData[_requestId] = _data;
        emit DataRetrieved(_requestId, _data);
    }

    function getStoredCID(bytes32 _requestId) public view returns (string memory) {
        return requestToCID[_requestId];
    }

    function getRetrievedData(bytes32 _requestId) public view returns (bytes memory) {
        return requestToData[_requestId];
    }

    function updateOracleDetails(address _oracle, bytes32 _jobId, uint256 _fee) public onlyOwner {
        oracle = _oracle;
        jobId = _jobId;
        fee = _fee;
    }

    function updateIPFSGatewayUrl(string memory _newUrl) public onlyOwner {
        ipfsGatewayUrl = _newUrl;
    }
}
