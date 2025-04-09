// Main contract implementation
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::errors::ContractError;
use crate::models::{Poll, VoteResults};

// Main contract struct that holds all state
pub struct VotingContract {
    // Mapping of poll_id to Poll struct
    polls: HashMap<u64, Poll>,
    // Mapping of poll_id to a map of wallet_address to vote_option
    votes: HashMap<u64, HashMap<String, u32>>,
    // Mapping of poll_id to VoteResults
    results: HashMap<u64, VoteResults>,
    // Poll counter for generating unique poll IDs
    poll_counter: u64,
    // Contract owner address
    owner: String,
}

impl VotingContract {
    // Initialize a new voting contract
    pub fn new(owner: String) -> Self {
        VotingContract {
            polls: HashMap::new(),
            votes: HashMap::new(),
            results: HashMap::new(),
            poll_counter: 0,
            owner,
        }
    }

    // Create a new poll
    pub fn create_poll(
        &mut self,
        creator: String, 
        title: String, 
        description: String, 
        options: Vec<String>, 
        start_time: u64, 
        end_time: u64
    ) -> Result<u64, ContractError> {
        // Validate inputs
        if options.len() < 2 {
            return Err(ContractError::InvalidOption);
        }
        
        if start_time >= end_time {
            return Err(ContractError::InvalidTimeRange);
        }
        
        // Generate a new unique poll ID
        let poll_id = self.poll_counter;
        self.poll_counter += 1;
        
        // Create the poll
        let poll = Poll {
            id: poll_id,
            title,
            description,
            options: options.clone(),
            creator,
            start_time,
            end_time,
            active: true,
        };
        
        // Initialize vote tracking for this poll
        self.polls.insert(poll_id, poll);
        self.votes.insert(poll_id, HashMap::new());
        
        // Initialize results for this poll
        let results = VoteResults::new(options.len());
        self.results.insert(poll_id, results);
        
        Ok(poll_id)
    }
    
    // Cast a vote in a poll
    pub fn vote(
        &mut self, 
        poll_id: u64, 
        wallet_address: String, 
        option_index: u32
    ) -> Result<(), ContractError> {
        // Check if poll exists
        let poll = match self.polls.get(&poll_id) {
            Some(p) => p,
            None => return Err(ContractError::PollNotFound),
        };
        
        // Check if poll is active
        if !poll.active {
            return Err(ContractError::PollNotActive);
        }
        
        // Check if voting period is valid
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        if current_time < poll.start_time {
            return Err(ContractError::PollNotActive);
        }
        
        if current_time > poll.end_time {
            return Err(ContractError::PollAlreadyEnded);
        }
        
        // Check if option is valid
        if option_index as usize >= poll.options.len() {
            return Err(ContractError::InvalidOption);
        }
        
        // Check if user has already voted
        let poll_votes = self.votes.get_mut(&poll_id).unwrap();
        if poll_votes.contains_key(&wallet_address) {
            return Err(ContractError::AlreadyVoted);
        }
        
        // Record the vote
        poll_votes.insert(wallet_address, option_index);
        
        // Update the results
        let results = self.results.get_mut(&poll_id).unwrap();
        let count = results.counts.get_mut(&option_index).unwrap();
        *count += 1;
        results.total_votes += 1;
        
        Ok(())
    }
    
    // Get poll information
    pub fn get_poll(&self, poll_id: u64) -> Result<&Poll, ContractError> {
        match self.polls.get(&poll_id) {
            Some(poll) => Ok(poll),
            None => Err(ContractError::PollNotFound),
        }
    }
    
    // Get poll results
    pub fn get_results(&self, poll_id: u64) -> Result<&VoteResults, ContractError> {
        match self.results.get(&poll_id) {
            Some(results) => Ok(results),
            None => Err(ContractError::PollNotFound),
        }
    }
    
    // Close a poll (only creator or owner can do this)
    pub fn close_poll(&mut self, poll_id: u64, caller: String) -> Result<(), ContractError> {
        let poll = match self.polls.get_mut(&poll_id) {
            Some(p) => p,
            None => return Err(ContractError::PollNotFound),
        };
        
        // Check if caller is authorized
        if poll.creator != caller && self.owner != caller {
            return Err(ContractError::Unauthorized);
        }
        
        poll.active = false;
        
        Ok(())
    }
    
    // Auto-close polls that have reached their end time
    pub fn process_expired_polls(&mut self) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        for (_, poll) in self.polls.iter_mut() {
            if poll.active && current_time > poll.end_time {
                poll.active = false;
            }
        }
    }
    
    // Get all active polls
    pub fn get_active_polls(&self) -> Vec<u64> {
        let mut active_polls = Vec::new();
        
        for (id, poll) in &self.polls {
            if poll.active {
                active_polls.push(*id);
            }
        }
        
        active_polls
    }
    
    // Get detailed vote results with percentage
    pub fn get_detailed_results(&self, poll_id: u64) -> Result<HashMap<String, (u64, f64)>, ContractError> {
        let poll = match self.polls.get(&poll_id) {
            Some(p) => p,
            None => return Err(ContractError::PollNotFound),
        };
        
        let results = match self.results.get(&poll_id) {
            Some(r) => r,
            None => return Err(ContractError::PollNotFound),
        };
        
        let mut detailed_results = HashMap::new();
        let total_votes = results.total_votes;
        
        for (option_idx, count) in &results.counts {
            let option_name = &poll.options[*option_idx as usize];
            let percentage = if total_votes > 0 {
                (*count as f64 / total_votes as f64) * 100.0
            } else {
                0.0
            };
            
            detailed_results.insert(option_name.clone(), (*count, percentage));
        }
        
        Ok(detailed_results)
    }
    
    // Check if an address has voted in a poll
    pub fn has_voted(&self, poll_id: u64, wallet_address: &str) -> Result<bool, ContractError> {
        let poll_votes = match self.votes.get(&poll_id) {
            Some(votes) => votes,
            None => return Err(ContractError::PollNotFound),
        };
        
        Ok(poll_votes.contains_key(wallet_address))
    }
}