// Integration tests for the Arch Voting Contract
use arch_voting_contract::{VotingContract, ContractError};

// Test the full voting workflow from creation to results
#[test]
fn test_voting_workflow() {
    // Initialize a new contract
    let mut contract = VotingContract::new("contract_owner".to_string());
    
    // Create a new poll
    let poll_id = contract.create_poll(
        "poll_creator".to_string(),
        "Favorite Color".to_string(),
        "Vote for your favorite color".to_string(),
        vec![
            "Red".to_string(),
            "Blue".to_string(),
            "Green".to_string(),
            "Yellow".to_string(),
        ],
        0, // start time (now)
        u64::MAX, // end time (far future)
    ).unwrap();
    
    // Cast votes from multiple wallets
    let wallets = vec![
        "wallet1", "wallet2", "wallet3", "wallet4", 
        "wallet5", "wallet6", "wallet7", "wallet8",
        "wallet9", "wallet10"
    ];
    
    // Distribute votes: 4 for Red, 3 for Blue, 2 for Green, 1 for Yellow
    contract.vote(poll_id, wallets[0].to_string(), 0).unwrap(); // Red
    contract.vote(poll_id, wallets[1].to_string(), 0).unwrap(); // Red
    contract.vote(poll_id, wallets[2].to_string(), 0).unwrap(); // Red
    contract.vote(poll_id, wallets[3].to_string(), 0).unwrap(); // Red
    
    contract.vote(poll_id, wallets[4].to_string(), 1).unwrap(); // Blue
    contract.vote(poll_id, wallets[5].to_string(), 1).unwrap(); // Blue
    contract.vote(poll_id, wallets[6].to_string(), 1).unwrap(); // Blue
    
    contract.vote(poll_id, wallets[7].to_string(), 2).unwrap(); // Green
    contract.vote(poll_id, wallets[8].to_string(), 2).unwrap(); // Green
    
    contract.vote(poll_id, wallets[9].to_string(), 3).unwrap(); // Yellow
    
    // Get detailed results
    let detailed_results = contract.get_detailed_results(poll_id).unwrap();
    
    // Verify counts
    let (red_count, red_percentage) = detailed_results.get("Red").unwrap();
    let (blue_count, blue_percentage) = detailed_results.get("Blue").unwrap();
    let (green_count, green_percentage) = detailed_results.get("Green").unwrap();
    let (yellow_count, yellow_percentage) = detailed_results.get("Yellow").unwrap();
    
    assert_eq!(*red_count, 4);
    assert_eq!(*blue_count, 3);
    assert_eq!(*green_count, 2);
    assert_eq!(*yellow_count, 1);
    
    // Verify percentages
    assert!((red_percentage - 40.0).abs() < 0.01);    // 4/10 = 40%
    assert!((blue_percentage - 30.0).abs() < 0.01);   // 3/10 = 30%
    assert!((green_percentage - 20.0).abs() < 0.01);  // 2/10 = 20%
    assert!((yellow_percentage - 10.0).abs() < 0.01); // 1/10 = 10%
    
    // Close the poll
    contract.close_poll(poll_id, "poll_creator".to_string()).unwrap();
    
    // Verify poll is closed
    let poll = contract.get_poll(poll_id).unwrap();
    assert_eq!(poll.active, false);
    
    // Attempt to vote after closure should fail
    let result = contract.vote(poll_id, "new_wallet".to_string(), 0);
    assert!(matches!(result, Err(ContractError::PollNotActive)));
    
    // Results should remain the same
    let final_results = contract.get_results(poll_id).unwrap();
    assert_eq!(final_results.total_votes, 10);
}

// Test edge cases with poll timing
#[test]
fn test_poll_timing() {
    let mut contract = VotingContract::new("owner".to_string());
    
    // Get current time (this is a simplification, real implementations would use blockchain time)
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // Create a poll that starts in the future
    let future_poll_id = contract.create_poll(
        "creator".to_string(),
        "Future Poll".to_string(),
        "This poll starts in the future".to_string(),
        vec!["Yes".to_string(), "No".to_string()],
        now + 1000, // Starts 1000 seconds in the future
        now + 2000, // Ends 2000 seconds in the future
    ).unwrap();
    
    // Voting should fail because poll hasn't started
    let result = contract.vote(future_poll_id, "wallet".to_string(), 0);
    assert!(matches!(result, Err(ContractError::PollNotActive)));
    
    // Create a poll that has already ended
    let ended_poll_id = contract.create_poll(
        "creator".to_string(),
        "Ended Poll".to_string(),
        "This poll has already ended".to_string(),
        vec!["Yes".to_string(), "No".to_string()],
        now - 2000, // Started 2000 seconds in the past
        now - 1000, // Ended 1000 seconds in the past
    ).unwrap();
    
    // Voting should fail because poll has ended
    let result = contract.vote(ended_poll_id, "wallet".to_string(), 0);
    assert!(matches!(result, Err(ContractError::PollAlreadyEnded)));
    
    // Process expired polls
    contract.process_expired_polls();
    
    // Verify the ended poll is now marked as inactive
    let ended_poll = contract.get_poll(ended_poll_id).unwrap();
    assert_eq!(ended_poll.active, false);
    
    // The future poll should still be active
    let future_poll = contract.get_poll(future_poll_id).unwrap();
    assert_eq!(future_poll.active, true);
}

// Test multiple polls with various configurations
#[test]
fn test_multiple_polls() {
    let mut contract = VotingContract::new("owner".to_string());
    
    // Create multiple polls
    let poll_ids = vec![
        contract.create_poll(
            "creator1".to_string(),
            "Binary Poll".to_string(),
            "Simple yes/no poll".to_string(),
            vec!["Yes".to_string(), "No".to_string()],
            0,
            u64::MAX,
        ).unwrap(),
        
        contract.create_poll(
            "creator2".to_string(),
            "Multiple Choice Poll".to_string(),
            "Poll with multiple options".to_string(),
            vec![
                "Option A".to_string(),
                "Option B".to_string(),
                "Option C".to_string(),
                "Option D".to_string(),
            ],
            0,
            u64::MAX,
        ).unwrap(),
        
        contract.create_poll(
            "creator3".to_string(),
            "Rating Poll".to_string(),
            "Rate from 1 to 5".to_string(),
            vec![
                "1 - Poor".to_string(),
                "2 - Fair".to_string(),
                "3 - Good".to_string(),
                "4 - Very Good".to_string(),
                "5 - Excellent".to_string(),
            ],
            0,
            u64::MAX,
        ).unwrap(),
    ];
    
    // Verify all polls were created
    assert_eq!(poll_ids.len(), 3);
    
    // Verify we can retrieve each poll
    for poll_id in &poll_ids {
        let poll = contract.get_poll(*poll_id).unwrap();
        assert!(poll.active);
        
        // Each poll should have the correct number of options
        match *poll_id {
            id if id == poll_ids[0] => assert_eq!(poll.options.len(), 2),  // Binary poll
            id if id == poll_ids[1] => assert_eq!(poll.options.len(), 4),  // Multiple choice
            id if id == poll_ids[2] => assert_eq!(poll.options.len(), 5),  // Rating poll
            _ => panic!("Unexpected poll ID"),
        }
    }
    
    // Cast votes on different polls
    contract.vote(poll_ids[0], "voter1".to_string(), 0).unwrap(); // Yes on binary poll
    contract.vote(poll_ids[0], "voter2".to_string(), 1).unwrap(); // No on binary poll
    
    contract.vote(poll_ids[1], "voter1".to_string(), 2).unwrap(); // Option C on multiple choice
    contract.vote(poll_ids[1], "voter3".to_string(), 1).unwrap(); // Option B on multiple choice
    
    contract.vote(poll_ids[2], "voter2".to_string(), 4).unwrap(); // 5-Excellent on rating poll
    contract.vote(poll_ids[2], "voter3".to_string(), 3).unwrap(); // 4-Very Good on rating poll
    
    // Check results for each poll
    let binary_results = contract.get_results(poll_ids[0]).unwrap();
    assert_eq!(binary_results.total_votes, 2);
    
    let multiple_results = contract.get_results(poll_ids[1]).unwrap();
    assert_eq!(multiple_results.total_votes, 2);
    
    let rating_results = contract.get_results(poll_ids[2]).unwrap();
    assert_eq!(rating_results.total_votes, 2);
    
    // Verify that a voter can vote in multiple polls but only once per poll
    assert!(contract.has_voted(poll_ids[0], "voter1").unwrap());
    assert!(contract.has_voted(poll_ids[1], "voter1").unwrap());
    assert!(!contract.has_voted(poll_ids[2], "voter1").unwrap());
    
    // Attempt to vote again should fail
    let result = contract.vote(poll_ids[0], "voter1".to_string(), 1);
    assert!(matches!(result, Err(ContractError::AlreadyVoted)));
}