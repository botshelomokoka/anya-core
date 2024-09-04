// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "@chainlink/contracts/src/v0.8/ChainlinkClient.sol";

/**
 * @title AIReview
 * @dev A contract to interact with an AI oracle for reviewing certain DAO operations
 */
contract AIReview is Ownable, ReentrancyGuard, ChainlinkClient {
    using Chainlink for Chainlink.Request;

    address public aiOracle;
    bytes32 private jobId;
    uint256 private fee;

    mapping(bytes32 => bool) public pendingRequests;
    mapping(bytes32 => bool) public approvedOperations;

    event ReviewRequested(bytes32 indexed requestId, address indexed target, string signature, bytes data);
    event ReviewCompleted(bytes32 indexed requestId, bool approved);

    constructor(address _aiOracle, address _link, bytes32 _jobId, uint256 _fee) {
        aiOracle = _aiOracle;
        setChainlinkToken(_link);
        jobId = _jobId;
        fee = _fee;
    }

    function requestReview(address target, string memory signature, bytes calldata data) public nonReentrant {
        require(msg.sender == owner(), "Only owner can request reviews");
        Chainlink.Request memory request = buildChainlinkRequest(jobId, address(this), this.fulfillReview.selector);
        
        request.add("target", Strings.toHexString(uint160(target), 20));
        request.add("signature", signature);
        request.add("data", Strings.toHexString(uint256(uint160(bytes20(data))), 32));
        
        bytes32 requestId = sendChainlinkRequestTo(aiOracle, request, fee);
        pendingRequests[requestId] = true;
        
        emit ReviewRequested(requestId, target, signature, data);
    }

    function fulfillReview(bytes32 _requestId, bool _approved) public recordChainlinkFulfillment(_requestId) {
        require(pendingRequests[_requestId], "Request not found");
        delete pendingRequests[_requestId];
        approvedOperations[_requestId] = _approved;
        
        emit ReviewCompleted(_requestId, _approved);
    }

    function isApproved(bytes32 requestId) public view returns (bool) {
        require(!pendingRequests[requestId], "Review is still pending");
        return approvedOperations[requestId];
    }

    function withdrawLink() public onlyOwner {
        LinkTokenInterface link = LinkTokenInterface(chainlinkTokenAddress());
        require(link.transfer(msg.sender, link.balanceOf(address(this))), "Unable to transfer");
    }
}

/**
 * @title FounderVesting
 * @dev A contract to manage vesting of ANYA tokens for founders
 */
contract FounderVesting is Ownable, ReentrancyGuard {
    using SafeMath for uint256;

    IERC20 public anyaToken;
    address public founder;
    uint256 public vestingStart;
    uint256 public vestingDuration;
    uint256 public totalVestingAmount;

    uint256 public totalVestedAmount;
    uint256 public lastClaimTime;

    event TokensClaimed(address indexed founder, uint256 amount);

    constructor(IERC20 _anyaToken, address _founder, uint256 _vestingStart, uint256 _vestingDuration, uint256 _totalVestingAmount) {
        require(_founder != address(0), "Invalid founder address");
        require(address(_anyaToken) != address(0), "Invalid token address");
        require(_vestingDuration > 0, "Vesting duration must be greater than 0");
        require(_totalVestingAmount > 0, "Total vesting amount must be greater than 0");

        anyaToken = _anyaToken;
        founder = _founder;
        vestingStart = _vestingStart;
        vestingDuration = _vestingDuration;
        totalVestingAmount = _totalVestingAmount;
        lastClaimTime = _vestingStart;
    }

    function claim() public nonReentrant {
        require(msg.sender == founder, "Only the founder can claim vested tokens");
        require(block.timestamp >= vestingStart, "Vesting has not yet started");

        uint256 vestedAmount = calculateVestedAmount();
        require(vestedAmount > 0, "No tokens are currently vested");

        totalVestedAmount = totalVestedAmount.add(vestedAmount);
        lastClaimTime = block.timestamp;

        require(anyaToken.transfer(founder, vestedAmount), "Token transfer failed");

        emit TokensClaimed(founder, vestedAmount);
    }

    function calculateVestedAmount() public view returns (uint256) {
        if (block.timestamp < vestingStart) {
            return 0;
        }

        uint256 elapsedTime = block.timestamp.sub(lastClaimTime);
        uint256 vestedAmount = totalVestingAmount.mul(elapsedTime).div(vestingDuration);

        uint256 remainingVesting = totalVestingAmount.sub(totalVestedAmount);
        return vestedAmount > remainingVesting ? remainingVesting : vestedAmount;
    }

    function emergencyWithdraw() public onlyOwner {
        uint256 balance = anyaToken.balanceOf(address(this));
        require(anyaToken.transfer(owner(), balance), "Token transfer failed");
    }
}
