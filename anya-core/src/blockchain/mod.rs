// Bitcoin module handles Bitcoin-specific blockchain operations
mod bitcoin;

// Lightning module handles Lightning Network operations
mod lightning;

// Stacks module handles Stacks blockchain operations
mod stacks;

pub use bitcoin::BitcoinCore;
pub use lightning::Lightning;
pub use stacks::Stacks;
