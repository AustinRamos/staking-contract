use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::StakingError::InvalidInstruction;

pub enum StakingInstruction{ 
    

    ///accounts expected:
    /// 
    /// 0. '[signer]' initializer
    /// 1. '[writable]' initializer_tokenA_account
    /// 2. '[writable]' tokA_vault
    /// 3    xTokA_to_recieve
    /// 4    rent sysvar
    /// 5.   token program

    DepositToVault{        
        vesting_period: u64,
        amount_to_deposit: u64,
    },

    ///accounts expected:
    ///  0. '[signer]' the xTokenA minting authority
    ///  1. '[writable]' the mint
    ///  2. [writable] the initializers main account to mint tokens to
    ///  3. '[writable]' the staking program account, holds (and eventually reward stuff)
    ///  4. '[writable]' the initializers xTokA account.
    ///  5. '[]' rent sysvar
    ///  6. '[]' the token program
    MintxTokA{
        amount_to_mint: u64,
    },
    
    ///0. '[signer]' initializer account main pubkey
    /// 1 [writable] temp token account that should be created prior to this instruction and owned by initializer
    /// tokenprogram
        DepositxTokA{
                amount: u64,
        },
   


    WithdrawFromVault{
        amount: u64,
        //eventually the reward/penalty calculation in state
    },
    /// Accounts expected:
   /// 2. '[writable]' the staking program account, it will hold all necessary info 
   /// 3. [writable] PDA temp token accnt to get tokens from(and eventuially close)
   /// 4 the pda account
   /// 5 xTokA mint
   /// 6 xTokA mint authority
    BurnxTokA{
        //would burn the pda account which owns the temp token account, so no need to know amount
    },
    SimpleStateSchange{}
}

impl StakingInstruction{
      pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::DepositToVault {
                vesting_period: Self::unpack_next_int(rest)?,
                amount_to_deposit: Self::unpack_next_int(rest.split_first().ok_or(InvalidInstruction)?.1)?,
            },
            1 => Self::MintxTokA {
                amount_to_mint: Self::unpack_next_int(rest.split_first().ok_or(InvalidInstruction)?.1)?,
            },
            2 => Self::DepositxTokA {
                amount: Self::unpack_next_int(rest.split_first().ok_or(InvalidInstruction)?.1)?,

            },
            3 => Self::WithdrawFromVault {
                amount: Self::unpack_next_int(rest.split_first().ok_or(InvalidInstruction)?.1)?,
            },
            4 => Self::BurnxTokA {
                amount: Self::unpack_next_int(rest.split_first().ok_or(InvalidInstruction)?.1)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    //unpacks vesting period and amount.
    fn unpack_next_int(input: &[u8]) -> Result<u64, ProgramError> {
        let vesting_period = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(vesting_period)
    }
    }
