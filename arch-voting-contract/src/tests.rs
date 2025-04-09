#[cfg(test)]
mod tests {
    use crate::contract::VotingContract;
    use crate::errors::ContractError;
    
    #[test]
    fn test_create_poll() {
        let mut contract = VotingContract::new("owner_address".to_string());
        
        let poll_id = contract.create_poll(
            "creator_address".to_string(),
            "Test Poll".to_string(),
            "Description of test poll".to_string(),
            vec!["Option 1".to_string(), "Option 2".to_string(), "Option 3".to_string()],
            100, // start time
            200, // end time
        ).unwrap();
        
        let poll = contract.get_poll(poll_id).unwrap();
        assert_eq!(poll.title, "Test Poll");
        assert_eq!(poll.options.len(), 3);
        assert_eq!(poll.creator, "creator_address");
        assert_eq!(poll.active, true);
    }
    
    #[test]
    fn test_invalid_poll_creation() {
        let mut contract = VotingContract::new("owner_address".to_string());
        
        // Test with invalid time range (start >= end)
        let result = contract.create_poll(
            "creator_address".to_string(),
            "Invalid Poll".to_string(),
            "Description".to_string(),
            vec!["Option 1".to_string(), "Option 2".to_string()],
            200, // start time
            100, // end time
        );
        assert!(matches!(result, Err(ContractError::InvalidTimeRange)));
        
        // Test with only one option
        let result = contract.create_poll(
            "creator_address".to_string(),
            "Invalid Poll".to_string(),
            "Description".to_string(),
            vec!["Option 1".to_string()], // Only one option
            100,
            200,
        );
        assert!(matches!(result, Err(ContractError::InvalidOption)));
    }
    
    #[test]
    fn test_voting() {
        let mut contract = VotingContract::new("owner_address".to_string());
        
        let poll_id = contract.create_poll(
            "creator_address".to_string(),
            "Test Poll".to_string(),
            "Description of test poll".to_string(),
            vec!["Option 1".to_string(), "Option 2".to_string(), "Option 3".to_string()],
            0, // start time in past
            u64::MAX, // end time in future
        ).unwrap();
        
        // Cast votes from different wallets
        contract.vote(poll_id, "wallet1".to_string(), 0).unwrap();
        contract.vote(poll_id, "wallet2".to_string(), 1).unwrap();
        contract.vote(poll_id, "wallet3".to_string(), 0).unwrap();
        
        // Check results
        let results = contract.get_results(poll_id).unwrap();
        assert_eq!(results.total_votes, 3);
        assert_eq!(*results.counts.get(&0).unwrap(), 2); // Option 1 got 2 votes
        assert_eq!(*results.counts.get(&1).unwrap(), 1); // Option 2 got 1 vote
        assert_eq!(*results.counts.get(&2).unwrap(), 0); // Option 3 got 0 votes
    }
    
    #[test]
    fn test_double_voting_prevention() {
        let mut contract = VotingContract::new("owner_address".to_string());
        
        let poll_id = contract.create_poll(
            "creator_address".to_string(),
            "Test Poll".to_string(),
            "Description of test poll".to_string(),
            vec!["Option 1".to_string(), "Option 2".to_string()],
            0, // start time in past
            u64::MAX, // end time in future
        ).unwrap();
        
        // First vote should succeed
        contract.vote(poll_id, "wallet1".to_string(), 0).unwrap();
        
        // Check that the wallet has voted
        assert!(contract.has_voted(poll_id, "wallet1").unwrap());
        
        // Second vote from same wallet should fail
        let result = contract.vote(poll_id, "wallet1".to_string(), 1);
        assert!(matches!(result, Err(ContractError::AlreadyVoted)));
        
        // Check that only one vote was counted
        let results = contract.get_results(poll_id).unwrap();
        assert_eq!(results.total_votes, 1);
    }
    
    #[test]
    fn test_poll_closure() {
        let mut contract = VotingContract::new("owner_address".to_string());
        
        let poll_id = contract.create_poll(
            "creator_address".to_string(),
            "Test Poll".to_string(),
            "Description of test poll".to_string(),
            vec!["Option 1".to_string(), "Option 2".to_string()],
            0, // start time in past
            u64::MAX, // end time in future
        ).unwrap();
        
        // Close the poll
        contract.close_poll(poll_id, "creator_address".to_string()).unwrap();
        
        // Verify poll is closed
        let poll = contract.get_poll(poll_id).unwrap();
        assert_eq!(poll.active, false);
        
        // Voting should fail now
        let result = contract.vote(poll_id, "wallet1".to_string(), 0);
        assert!(matches!(result, Err(ContractError::PollNotActive)));
    }
    
    #[test]
    fn test_detailed_results() {
        let mut contract = VotingContract::new("owner_address".to_string());
        
        let poll_id = contract.create_poll(
            "creator_address".to_string(),
            "Test Poll".to_string(),
            "Description of test poll".to_string(),
            vec!["Option 1".to_string(), "Option 2".to_string()],
            0, // start time in past
            u64::MAX, // end time in future
        ).unwrap();
        
        // Cast votes
        contract.vote(poll_id, "wallet1".to_string(), 0).unwrap();
        contract.vote(poll_id, "wallet2".to_string(), 0).unwrap();
        contract.vote(poll_id, "wallet3".to_string(), 1).unwrap();
        
        // Get detailed results
        let detailed = contract.get_detailed_results(poll_id).unwrap();
        
        // Check counts and percentages
        let (option1_count, option1_percentage) = detailed.get("Option 1").unwrap();
        let (option2_count, option2_percentage) = detailed.get("Option 2").unwrap();
        
        assert_eq!(*option1_count, 2);
        assert_eq!(*option2_count, 1);
        
        // Check percentages (2/3 ≈ 66.67% and 1/3 ≈ 33.33%)
        assert!((option1_percentage - 66.67).abs() < 0.01);
        assert!((option2_percentage - 33.33).abs() < 0.01);
    }
    
    #[test]
    fn test_unauthorized_poll_closure() {
        let mut contract = VotingContract::new("owner_address".to_string());
        
        let poll_id = contract.create_poll(
            "creator_address".to_string(),
            "Test Poll".to_string(),
            "Description of test poll".to_string(),
            vec!["Option 1".to_string(), "Option 2".to_string()],
            0,
            u64::MAX,
        ).unwrap();
        
        // Try to close with an unauthorized address
        let result = contract.close_poll(poll_id, "unauthorized_address".to_string());
        assert!(matches!(result, Err(ContractError::Unauthorized)));
        
        // Poll should still be active
        let poll = contract.get_poll(poll_id).unwrap();
        assert_eq!(poll.active, true);
        
        // Owner should be able to close any poll
        contract.close_poll(poll_id, "owner_address".to_string()).unwrap();
        
        // Poll should now be closed
        let poll = contract.get_poll(poll_id).unwrap();
        assert_eq!(poll.active, false);
    }
    
    #[test]
    fn test_active_polls_listing() {
        let mut contract = VotingContract::new("owner_address".to_string());
        
        // Create three polls
        let poll_id1 = contract.create_poll(
            "creator1".to_string(),
            "Poll 1".to_string(),
            "Description 1".to_string(),
            vec!["Yes".to_string(), "No".to_string()],
            0,
            u64::MAX,
        ).unwrap();
        
        let poll_id2 = contract.create_poll(
            "creator2".to_string(),
            "Poll 2".to_string(),
            "Description 2".to_string(),
            vec!["Option A".to_string(), "Option B".to_string()],
            0,
            u64::MAX,
        ).unwrap();
        
        let poll_id3 = contract.create_poll(
            "creator3".to_string(),
            "Poll 3".to_string(),
            "Description 3".to_string(),
            vec!["Red".to_string(), "Blue".to_string()],
            0,
            u64::MAX,
        ).unwrap();
        
        // All polls should be active
        let active_polls = contract.get_active_polls();
        assert_eq!(active_polls.len(), 3);
        assert!(active_polls.contains(&poll_id1));
        assert!(active_polls.contains(&poll_id2));
        assert!(active_polls.contains(&poll_id3));
        
        // Close one poll
        contract.close_poll(poll_id2, "creator2".to_string()).unwrap();
        
        // Now only two polls should be active
        let active_polls = contract.get_active_polls();
        assert_eq!(active_polls.len(), 2);
        assert!(active_polls.contains(&poll_id1));
        assert!(!active_polls.contains(&poll_id2));
        assert!(active_polls.contains(&poll_id3));
    }
}