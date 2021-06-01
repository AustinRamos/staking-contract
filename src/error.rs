use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum StakingError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("NotRentExempt")]
    NotRentExempt,
    #[error("Deserialized account is not an SPL Token mint")]
    ExpectedMint,
    #[error("The provided token program does not match the token program expected by the program")]
    IncorrectTokenProgramId,
    #[error("The provided token balance is zero")]
    TokenBalanceZero,
    #[error("must have a matching TokA deposit for any attempted xTokA mint.")]
    NoMatchingDeposit,
    #[error("you can not mint more than you have locked in the protocol.")]
    MintAmountExceedsLockedValue,
    #[error("The provided token account does not match the token account expected by the program")]
    NotExpectedTokenAccount
}


impl From<StakingError> for ProgramError {
    fn from(e: StakingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}