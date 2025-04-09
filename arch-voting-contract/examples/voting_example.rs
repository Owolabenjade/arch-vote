// Example usage of the Arch Voting Contract
use arch_voting_contract::VotingContract;

fn main() {
    // Create a new contract with the owner's wallet address
    let mut contract = VotingContract::new("owner_wallet_address".to_string());
    
    println!("=== Arch Voting Contract Example ===");
    println!("Initializing contract...");
    
    // Create a community governance poll
    let governance_poll_id = contract.create_poll(
        "governance_committee".to_string(),
        "Community Treasury Allocation".to_string(),
        "How should we allocate the community treasury funds?".to_string(),
        vec![
            "Fund developer grants".to_string(),
            "Improve protocol security".to_string(),
            "Marketing and growth".to_string(),
            "Save for future use".to_string(),
        ],
        // Current time + 1 day for start
        current_time() + 86400,
        // Current time + 8 days for end (1 week voting period)
        current_time() + (86400 * 8),
    ).unwrap();
    
    println!("Created governance poll with ID: {}", governance_poll_id);
    
    // Create a feature preference poll
    let feature_poll_id = contract.create_poll(
        "product_team".to_string(),
        "Next Feature Priority".to_string(),
        "Which feature should we prioritize next?".to_string(),
        vec![
            "Mobile wallet integration".to_string(),
            "Cross-chain compatibility".to_string(),
            "Advanced analytics dashboard".to_string(),
            "Fiat on-ramp".to_string(),
            "DAO governance tools".to_string(),
        ],
        // Start immediately
        current_time(),
        // End in 3 days
        current_time() + (86400 * 3),
    ).unwrap();
    
    println!("Created feature poll with ID: {}", feature_poll_id);
    
    // Simulate some votes on the feature poll
    let voters = vec![
        "wallet1", "wallet2", "wallet3", "wallet4", 
        "wallet5", "wallet6", "wallet7", "wallet8",
    ];
    
    println!("\nSimulating votes on feature poll:");
    
    // Cast votes with different choices
    contract.vote(feature_poll_id, voters[0].to_string(), 0).unwrap();
    println!("Voter {} voted for 'Mobile wallet integration'", voters[0]);
    
    contract.vote(feature_poll_id, voters[1].to_string(), 0).unwrap();
    println!("Voter {} voted for 'Mobile wallet integration'", voters[1]);
    
    contract.vote(feature_poll_id, voters[2].to_string(), 1).unwrap();
    println!("Voter {} voted for 'Cross-chain compatibility'", voters[2]);
    
    contract.vote(feature_poll_id, voters[3].to_string(), 2).unwrap();
    println!("Voter {} voted for 'Advanced analytics dashboard'", voters[3]);
    
    contract.vote(feature_poll_id, voters[4].to_string(), 3).unwrap();
    println!("Voter {} voted for 'Fiat on-ramp'", voters[4]);
    
    contract.vote(feature_poll_id, voters[5].to_string(), 4).unwrap();
    println!("Voter {} voted for 'DAO governance tools'", voters[5]);
    
    contract.vote(feature_poll_id, voters[6].to_string(), 0).unwrap();
    println!("Voter {} voted for 'Mobile wallet integration'", voters[6]);
    
    contract.vote(feature_poll_id, voters[7].to_string(), 1).unwrap();
    println!("Voter {} voted for 'Cross-chain compatibility'", voters[7]);
    
    // Display current results
    println!("\nCurrent Feature Poll Results:");
    let detailed_results = contract.get_detailed_results(feature_poll_id).unwrap();
    
    // Sort results by vote count (descending)
    let mut sorted_results: Vec<(&String, &(u64, f64))> = detailed_results.iter().collect();
    sorted_results.sort_by(|a, b| b.1.0.cmp(&a.1.0));
    
    for (option, (count, percentage)) in sorted_results {
        println!("{}: {} votes ({:.2}%)", option, count, percentage);
    }
    
    // Demonstrate poll closure
    println!("\nClosing the feature poll...");
    contract.close_poll(feature_poll_id, "product_team".to_string()).unwrap();
    
    let poll = contract.get_poll(feature_poll_id).unwrap();
    println!("Poll '{}' is now {}", poll.title, if poll.active { "active" } else { "closed" });
    
    // Show active polls
    println!("\nActive polls: {:?}", contract.get_active_polls());
}

// Helper function to get current time (in a real implementation, this would use blockchain time)
fn current_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}