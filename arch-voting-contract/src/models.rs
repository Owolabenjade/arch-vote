// Data models for the voting contract
use std::collections::HashMap;

// Represents a single poll
pub struct Poll {
    pub id: u64,
    pub title: String,
    pub description: String,
    // List of options that users can vote for
    pub options: Vec<String>,
    // Creator of the poll
    pub creator: String,
    // Unix timestamp when voting starts
    pub start_time: u64,
    // Unix timestamp when voting ends
    pub end_time: u64,
    // Whether the poll is active
    pub active: bool,
}

// Results of a poll
pub struct VoteResults {
    // Mapping of option_index to vote count
    pub counts: HashMap<u32, u64>,
    // Total number of votes cast
    pub total_votes: u64,
}

impl VoteResults {
    // Create a new empty results object
    pub fn new(option_count: usize) -> Self {
        let mut counts = HashMap::new();
        
        // Initialize all option counts to zero
        for i in 0..option_count {
            counts.insert(i as u32, 0);
        }
        
        VoteResults {
            counts,
            total_votes: 0,
        }
    }
}