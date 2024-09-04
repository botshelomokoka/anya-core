// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/governance/TimelockController.sol";
import "./AIOracle.sol";
import "./interfaces/IAIOracle.sol";

/**
 * @title TimelockControllerWithAI
 * @dev Extends OpenZeppelin's TimelockController to incorporate AI review for certain operations
 */
contract TimelockControllerWithAI is TimelockController {
    IAIOracle public aiOracle;

    constructor(
        uint256 minDelay,
        address[] memory proposers,
        address[] memory executors,
        address _aiOracle
    ) TimelockController(minDelay, proposers, executors) {
        aiOracle = IAIOracle(_aiOracle);
    }

    function queueTransaction(
        address target,
        uint256 value,
        string calldata signature,
        bytes calldata data,
        uint256 eta
    ) public override returns (bytes32) {
        bytes32 txHash = super.queueTransaction(target, value, signature, data, eta);

        if (shouldTriggerAIReview(target, signature)) {
            aiOracle.requestReview(txHash, target, signature, data);
        }

        return txHash;
    }

    function executeTransaction(
        address target,
        uint256 value,
        string calldata signature,
        bytes calldata data,
        uint256 eta
    ) public payable override returns (bytes memory) {
        if (shouldTriggerAIReview(target, signature)) {
            bytes32 txHash = hashOperation(target, value, signature, data, eta);
            require(aiOracle.isApproved(txHash), "AI review not approved or pending");
        }

        return super.executeTransaction(target, value, signature, data, eta);
    }

    function shouldTriggerAIReview(address target, string memory signature) internal view returns (bool) {
        // Implement logic to determine if AI review is needed
        // For example, trigger review for high-value transactions or specific function signatures
        return true; // Placeholder - replace with actual logic
    }

    function setAIOracle(address _aiOracle) external onlyRole(TIMELOCK_ADMIN_ROLE) {
        aiOracle = IAIOracle(_aiOracle);
    }
}
