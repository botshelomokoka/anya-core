// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";

/**
 * @title Reputation
 * @author Botshelo Mokoka
 * @dev A contract to manage reputation scores for Anya DAO members
 */
contract Reputation is Ownable {
    using SafeMath for uint256;

    mapping(address => uint256) public reputationScores;

    uint256 public constant MAX_REPUTATION = 100;
    uint256 public constant MIN_REPUTATION = 0;

    event ReputationUpdated(address indexed member, uint256 newScore);

    /**
     * @dev Increases the reputation score of a member
     * @param member The address of the member
     * @param amount The amount to increase the reputation by
     */
    function increaseReputation(address member, uint256 amount) external onlyOwner {
        uint256 newScore = reputationScores[member].add(amount);
        if (newScore > MAX_REPUTATION) {
            newScore = MAX_REPUTATION;
        }
        reputationScores[member] = newScore;
        emit ReputationUpdated(member, newScore);
    }

    /**
     * @dev Decreases the reputation score of a member
     * @param member The address of the member
     * @param amount The amount to decrease the reputation by
     */
    function decreaseReputation(address member, uint256 amount) external onlyOwner {
        uint256 newScore = reputationScores[member].sub(amount, "Reputation cannot be negative");
        if (newScore < MIN_REPUTATION) {
            newScore = MIN_REPUTATION;
        }
        reputationScores[member] = newScore;
        emit ReputationUpdated(member, newScore);
    }

    /**
     * @dev Gets the reputation score of a member
     * @param member The address of the member
     * @return The reputation score of the member
     */
    function getReputation(address member) external view returns (uint256) {
        return reputationScores[member];
    }
}
