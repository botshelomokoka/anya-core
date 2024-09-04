// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Snapshot.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/draft-ERC20Permit.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Votes.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";

/**
 * @title AnyaToken
 * @author Botshelo Mokoka
 * @notice ERC20 token with voting and snapshot capabilities for the Anya DAO
 * @dev Implements OpenZeppelin's ERC20 extensions for enhanced functionality
 */
contract AnyaToken is ERC20, ERC20Burnable, ERC20Snapshot, AccessControl, ERC20Permit, ERC20Votes {
    using SafeMath for uint256;

    bytes32 public constant SNAPSHOT_ROLE = keccak256("SNAPSHOT_ROLE");
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    uint256 private constant MAX_SUPPLY = 1_000_000_000 * 10**18; // 1 billion tokens

    /**
     * @dev Constructor that gives the msg.sender all of the default admin roles.
     */
    constructor() ERC20("Anya Token", "ANYA") ERC20Permit("Anya Token") {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(SNAPSHOT_ROLE, msg.sender);
        _grantRole(MINTER_ROLE, msg.sender);
        _mint(msg.sender, 100_000_000 * 10**18); // Mint initial 100 million tokens to deployer
    }

    /**
     * @dev Creates a new snapshot of the current token state.
     */
    function snapshot() public onlyRole(SNAPSHOT_ROLE) {
        _snapshot();
    }

    /**
     * @dev Mints new tokens. Can only be called by accounts with the MINTER_ROLE.
     * @param to The address that will receive the minted tokens
     * @param amount The amount of tokens to mint
     */
    function mint(address to, uint256 amount) public onlyRole(MINTER_ROLE) {
        require(totalSupply().add(amount) <= MAX_SUPPLY, "AnyaToken: Max supply exceeded");
        _mint(to, amount);
    }

    /**
     * @dev Internal function to mint tokens. Overridden to update votes.
     */
    function _mint(address to, uint256 amount) internal override(ERC20, ERC20Votes) {
        super._mint(to, amount);
    }

    /**
     * @dev Internal function to burn tokens. Overridden to update votes.
     */
    function _burn(address account, uint256 amount) internal override(ERC20, ERC20Votes) {
        super._burn(account, amount);
    }

    /**
     * @dev Internal function for token transfers. Overridden to update votes.
     */
    function _afterTokenTransfer(address from, address to, uint256 amount) internal override(ERC20, ERC20Snapshot, ERC20Votes) {
        super._afterTokenTransfer(from, to, amount);
    }

    /**
     * @dev Overridden to prioritize Votes functionality over Snapshots.
     */
    function _beforeTokenTransfer(address from, address to, uint256 amount) internal override(ERC20, ERC20Snapshot) {
        super._beforeTokenTransfer(from, to, amount);
    }

    /**
     * @dev Returns the current total supply of tokens.
     */
    function totalSupply() public view override(ERC20, ERC20Snapshot) returns (uint256) {
        return super.totalSupply();
    }
}