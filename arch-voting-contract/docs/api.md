# Arch Voting Contract API Documentation

This document provides detailed information about all available methods in the Arch Voting Contract.

## Core Types

### Poll

Represents a single voting poll:

```rust
pub struct Poll {
    pub id: u64,               // Unique poll identifier
    pub title: String,         // Poll title
    pub description: String,   // Poll description
    pub options: Vec<String>,  // Voting options
    pub creator: String,       // Address of poll creator
    pub start_time: u64,       // Unix timestamp when voting starts
    pub end_time: u64,         // Unix timestamp when voting ends
    pub active: bool,          // Whether the poll is currently active
}
```

### VoteResults

Contains the results of a poll:

```rust
pub struct VoteResults {
    pub counts: HashMap<u32, u64>,  // Mapping of option_index to vote count
    pub total_votes: u64,           // Total number of votes cast
}
```

### ContractError

Possible error types:

```rust
pub enum ContractError {
    Unauthorized,       // When caller doesn't have permission
    PollNotFound,       // When referenced poll doesn't exist
    PollNotActive,      // When poll is not in active state
    PollAlreadyEnded,   // When trying to vote after end time
    PollNotEnded,       // When trying to finalize before end time
    InvalidOption,      // When option index is out of bounds
    AlreadyVoted,       // When wallet has already voted
    InvalidTimeRange,   // When start_time >= end_time
}
```

## Contract Methods

### Constructor

```rust
pub fn new(owner: String) -> Self
```

Creates a new voting contract instance with the specified owner address.

**Parameters:**
- `owner`: The wallet address of the contract owner

**Returns:**
- A new `VotingContract` instance

### Create Poll

```rust
pub fn create_poll(
    &mut self,
    creator: String, 
    title: String, 
    description: String, 
    options: Vec<String>, 
    start_time: u64, 
    end_time: u64
) -> Result<u64, ContractError>
```

Creates a new poll with the specified parameters.

**Parameters:**
- `creator`: Wallet address of the poll creator
- `title`: Title of the poll
- `description`: Description of the poll
- `options`: Vector of voting options as strings
- `start_time`: Unix timestamp when voting begins
- `end_time`: Unix timestamp when voting ends

**Returns:**
- `Ok(poll_id)`: The ID of the newly created poll
- `Err(ContractError)`: An error if creation fails

**Possible Errors:**
- `InvalidOption`: If fewer than 2 options are provided
- `InvalidTimeRange`: If start_time >= end_time

### Vote

```rust
pub fn vote(
    &mut self, 
    poll_id: u64, 
    wallet_address: String, 
    option_index: u32
) -> Result<(), ContractError>
```

Casts a vote in a poll.

**Parameters:**
- `poll_id`: ID of the poll to vote in
- `wallet_address`: Address of the voter
- `option_index`: Index of the chosen option

**Returns:**
- `Ok(())`: If the vote was successful
- `Err(ContractError)`: An error if voting fails

**Possible Errors:**
- `PollNotFound`: If the poll ID doesn't exist
- `PollNotActive`: If the poll is not active
- `PollAlreadyEnded`: If the poll has ended
- `InvalidOption`: If the option index is invalid
- `AlreadyVoted`: If the wallet has already voted in this poll

### Get Poll

```rust
pub fn get_poll(&self, poll_id: u64) -> Result<&Poll, ContractError>
```

Retrieves information about a poll.

**Parameters:**
- `poll_id`: ID of the poll to retrieve

**Returns:**
- `Ok(&Poll)`: Reference to the poll if found
- `Err(ContractError)`: An error if poll doesn't exist

**Possible Errors:**
- `PollNotFound`: If the poll ID doesn't exist

### Get Results

```rust
pub fn get_results(&self, poll_id: u64) -> Result<&VoteResults, ContractError>
```

Gets the current results of a poll.

**Parameters:**
- `poll_id`: ID of the poll

**Returns:**
- `Ok(&VoteResults)`: Reference to the results if found
- `Err(ContractError)`: An error if poll doesn't exist

**Possible Errors:**
- `PollNotFound`: If the poll ID doesn't exist

### Close Poll

```rust
pub fn close_poll(&mut self, poll_id: u64, caller: String) -> Result<(), ContractError>
```

Closes a poll manually.

**Parameters:**
- `poll_id`: ID of the poll to close
- `caller`: Address of the caller

**Returns:**
- `Ok(())`: If successful
- `Err(ContractError)`: An error if operation fails

**Possible Errors:**
- `PollNotFound`: If the poll ID doesn't exist
- `Unauthorized`: If caller is neither the poll creator nor contract owner

### Process Expired Polls

```rust
pub fn process_expired_polls(&mut self)
```

Automatically closes all polls that have reached their end time.

**Parameters:** None

**Returns:** None

### Get Active Polls

```rust
pub fn get_active_polls(&self) -> Vec<u64>
```

Returns a list of all active poll IDs.

**Parameters:** None

**Returns:**
- `Vec<u64>`: Vector of active poll IDs

### Get Detailed Results

```rust
pub fn get_detailed_results(&self, poll_id: u64) -> Result<HashMap<String, (u64, f64)>, ContractError>
```

Gets detailed results with vote counts and percentages for each option.

**Parameters:**
- `poll_id`: ID of the poll

**Returns:**
- `Ok(HashMap<String, (u64, f64)>)`: Mapping of option string to (count, percentage)
- `Err(ContractError)`: An error if operation fails

**Possible Errors:**
- `PollNotFound`: If the poll ID doesn't exist

### Has Voted

```rust
pub fn has_voted(&self, poll_id: u64, wallet_address: &str) -> Result<bool, ContractError>
```

Checks if a wallet has already voted in a poll.

**Parameters:**
- `poll_id`: ID of the poll
- `wallet_address`: Address to check

**Returns:**
- `Ok(bool)`: True if wallet has voted, false otherwise
- `Err(ContractError)`: An error if operation fails

**Possible Errors:**
- `PollNotFound`: If the poll ID doesn't exist