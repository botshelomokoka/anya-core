// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";

/**
 * @title FounderVesting
 * @author Botshelo Mokoka
 * @notice A contract for vesting Anya tokens for founders over a specified period
 * @dev Implements a linear vesting schedule with a cliff
 */
contract FounderVesting is Ownable, ReentrancyGuard {
    using SafeMath for uint256;

    IERC20 public immutable anyaToken;
    
    struct VestingSchedule {
        uint256 totalAmount;
        uint256 startTime;
        uint256 cliffDuration;
        uint256 vestingDuration;
        uint256 releasedAmount;
    }

    mapping(address => VestingSchedule) public vestingSchedules;

    uint256 public constant VESTING_DURATION = 4 years;
    uint256 public constant CLIFF_DURATION = 1 years;
    uint256 public constant FOUNDER_ALLOCATION = 200_000_000 * 10**18; // 200 million tokens

    event TokensVested(address indexed beneficiary, uint256 amount);
    event VestingScheduleCreated(address indexed beneficiary, uint256 totalAmount, uint256 startTime, uint256 cliffDuration, uint256 vestingDuration);

    constructor(address _anyaToken) {
        anyaToken = IERC20(_anyaToken);
    }

    function createFounderVestingSchedule(address _founder) external onlyOwner {
        require(_founder != address(0), "Founder cannot be zero address");
        require(vestingSchedules[_founder].totalAmount == 0, "Vesting schedule already exists");

        uint256 startTime = block.timestamp;
        
        vestingSchedules[_founder] = VestingSchedule({
            totalAmount: FOUNDER_ALLOCATION,
            startTime: startTime,
            cliffDuration: CLIFF_DURATION,
            vestingDuration: VESTING_DURATION,
            releasedAmount: 0
        });

        emit VestingScheduleCreated(_founder, FOUNDER_ALLOCATION, startTime, CLIFF_DURATION, VESTING_DURATION);
    }

    function release() external nonReentrant {
        VestingSchedule storage schedule = vestingSchedules[msg.sender];
        require(schedule.totalAmount > 0, "No vesting schedule found");

        uint256 vestedAmount = _calculateVestedAmount(schedule);
        uint256 releaseAmount = vestedAmount.sub(schedule.releasedAmount);
        require(releaseAmount > 0, "No tokens available for release");

        schedule.releasedAmount = schedule.releasedAmount.add(releaseAmount);
        require(anyaToken.transfer(msg.sender, releaseAmount), "Token transfer failed");

        emit TokensVested(msg.sender, releaseAmount);
    }

    function _calculateVestedAmount(VestingSchedule memory _schedule) internal view returns (uint256) {
        if (block.timestamp < _schedule.startTime.add(_schedule.cliffDuration)) {
            return 0;
        } else if (block.timestamp >= _schedule.startTime.add(_schedule.vestingDuration)) {
            return _schedule.totalAmount;
        } else {
            return _schedule.totalAmount.mul(block.timestamp.sub(_schedule.startTime.add(_schedule.cliffDuration)))
                .div(_schedule.vestingDuration.sub(_schedule.cliffDuration));
        }
    }

    function getVestedAmount(address _beneficiary) external view returns (uint256) {
        VestingSchedule memory schedule = vestingSchedules[_beneficiary];
        return _calculateVestedAmount(schedule);
    }

    function getReleaseableAmount(address _beneficiary) external view returns (uint256) {
        VestingSchedule memory schedule = vestingSchedules[_beneficiary];
        uint256 vestedAmount = _calculateVestedAmount(schedule);
        return vestedAmount.sub(schedule.releasedAmount);
    }
}

