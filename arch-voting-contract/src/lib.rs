// Main entry point for the voting contract library

// Re-exports
pub mod contract;
pub mod models;
pub mod errors;

pub use contract::VotingContract;
pub use models::{Poll, VoteResults};
pub use errors::ContractError;