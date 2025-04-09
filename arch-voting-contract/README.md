# Arch Voting Contract

A decentralized voting smart contract for the Arch Ecosystem that enables users to create polls, cast votes securely, and view transparent results.

## 🚀 Features

- **Poll Creation**: Create customizable polls with multiple options
- **Secure Voting**: One vote per wallet address with on-chain verification
- **Real-time Results**: Instant vote tallying and detailed results with percentages
- **Time-bound Voting**: Automatic poll closure at the end of voting periods
- **Access Control**: Only poll creators and contract owners can perform administrative actions

## 📋 Technical Implementation

The contract implements all required specifications:

- **Secure Access Control**: Poll creators and contract owners have specific administrative privileges
- **Double-voting Prevention**: Built-in verification to ensure each wallet can only vote once per poll
- **Gas Efficiency**: Optimized data structures to minimize transaction costs
- **Real-time Tallying**: Constant-time complexity for vote counting regardless of poll size

## 🔧 Usage

### Creating a New Poll

```rust
let poll_id = contract.create_poll(
    creator_address,
    "Poll Title",
    "Poll Description",
    vec!["Option 1", "Option 2", "Option 3"],
    start_timestamp,
    end_timestamp
);
```

### Casting a Vote

```rust
contract.vote(poll_id, wallet_address, option_index);
```

### Viewing Results

```rust
// Basic results
let results = contract.get_results(poll_id);

// Detailed results with percentages
let detailed = contract.get_detailed_results(poll_id);
```

### Poll Management

```rust
// Close a poll manually
contract.close_poll(poll_id, caller_address);

// Process all expired polls
contract.process_expired_polls();

// Get all active polls
let active_polls = contract.get_active_polls();
```

## 🧪 Testing

The contract includes a comprehensive test suite:

- **Unit Tests**: Tests for individual functions and components
- **Integration Tests**: End-to-end workflow tests
- **Edge Cases**: Tests for various corner cases and error conditions

Run the tests with:

```bash
cargo test
```

## 📊 Example

See the `examples/voting_example.rs` file for a complete usage example demonstrating:

- Creating multiple polls with different configurations
- Casting votes from different wallet addresses
- Displaying real-time results with percentages
- Closing polls and checking their status

## 🔒 Security Considerations

- The contract verifies timestamps to ensure votes are only cast during the active voting period
- Double-voting is prevented through comprehensive wallet tracking
- Access control is strictly enforced for administrative actions
- All errors are handled explicitly with descriptive error types

## 🏗️ Future Extensions

Possible enhancements for future versions:

- Delegation of voting power
- Weighted voting based on token holdings
- Secret ballots with zero-knowledge proofs
- Integration with governance frameworks

## 🧩 Implementation Details

The contract is structured with clean separation of concerns:

- `models.rs`: Data structures for polls and results
- `errors.rs`: Error types for all possible failure conditions
- `contract.rs`: Core contract logic and state management
- `tests.rs`: Comprehensive unit tests

This modular design enhances maintainability and allows for future extensions.