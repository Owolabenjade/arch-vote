// Custom errors for the voting contract

#[derive(Debug)]
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