use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
    program::invoke
};

use crate::{instruction::StakingInstruction, error::StakingError, state::Staking};
use spl_token::state::Account as TokenAccount;
pub struct Processor;
impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = StakingInstruction::unpack(instruction_data)?;

        match instruction {
            StakingInstruction::DepositToVault { vesting_period, amount_to_deposit} => {
                msg!("Instruction: Deposit to Vault");
                Self::process_deposit(accounts, vesting_period, amount_to_deposit, program_id)
            },
            StakingInstruction::MintxTokA {amount_to_mint} => {
                msg!("Instruction: MintxTokA");
                Self::process_mint(accounts, amount_to_mint, program_id)
            },
            StakingInstruction::DepositxTokA {amount} => {
                msg!("Instruction: DepositxTokA");
                Self::process_mint(accounts, amount, program_id)
            },
            StakingInstruction::WithdrawFromVault {amount} => {
                msg!("Instruction: WithdrawFromVault");
                Self::process_mint(accounts, amount, program_id)
            },
            StakingInstruction::BurnxTokA {amount} => {
                msg!("Instruction: BurnxToA");
                Self::process_mint(accounts, amount, program_id)
            },
        }
    }

        fn process_deposit(
        accounts: &[AccountInfo],
        vesting_period: u64,
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;

        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let initializer_tokenA_account = next_account_info(account_info_iter)?;
        let initializer_tokenA_account_info = TokenAccount::unpack(&initializer_tokenA_account.data.borrow())?;
      if initializer_tokenA_account_info.amount==0 {
          return Err(StakingError::TokenBalanceZero.into());
      }
      
      msg!("Initializer_TokenA account owner: {} " ,initializer_tokenA_account_info.owner );


        let xtokenA_to_receive_account = next_account_info(account_info_iter)?;
        
        if *xtokenA_to_receive_account.owner != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

  
        let staking_account = next_account_info(account_info_iter)?;


        let tokA_vault = next_account_info(account_info_iter)?;
  

         msg!("TokA_Vault.key: {} " ,tokA_vault.key);

         let tokA_vault_info = TokenAccount::unpack(&tokA_vault.data.borrow())?;

msg!(" tokA_Vault.owner {}", tokA_vault_info.owner);


msg!("tokA_Vault.mint {}", tokA_vault_info.mint);


        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

if !rent.is_exempt(staking_account.lamports(), staking_account.data_len()) {
    return Err(StakingError::NotRentExempt.into());
}
let mut staking_info = Staking::unpack_unchecked(&staking_account.data.borrow())?;

/**this is how i can enforce that this  the right transactions happen after one another*/
if staking_info.is_initialized(){
    return Err(ProgramError::AccountAlreadyInitialized);
} 

staking_info.vesting_period = vesting_period;
staking_info.amount_currently_locked = staking_info.amount_currently_locked+amount; 
staking_info.is_initialized = true;
staking_info.initializer_token_to_receive_account_pubkey = *xtokenA_to_receive_account.key;

Staking::pack(staking_info, &mut staking_account.data.borrow_mut())?;

//let (pda, _bump_seed) = Pubkey::find_program_address(&[b"tokenAvault"], program_id);

 let token_program = next_account_info(account_info_iter)?;

msg!("making instruction to transfer {} coins to vault {}", amount,tokA_vault.key,);

let transfer_tokA_to_vault_ix = spl_token::instruction::transfer(
    token_program.key,
    initializer_tokenA_account.key,
    tokA_vault.key,
    initializer.key, 
    &[&initializer.key],
    amount
)?;

msg!("Calling the token program transfer TokenA to vault...");

invoke(
    &transfer_tokA_to_vault_ix,
    &[
        initializer_tokenA_account.clone(),
        tokA_vault.clone(), 
        initializer.clone(),
        token_program.clone(),
    ],
)?;
        Ok(())

    }


    fn process_mint(
        accounts: &[AccountInfo],
        amount_to_mint: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let xTokA_minting_authority = next_account_info(account_info_iter)?;
        let xTokA_mint = next_account_info(account_info_iter)?;
        let initializer= next_account_info(account_info_iter)?;

        let staking_account = next_account_info(account_info_iter)?;

        let initializer_xTokA_accnt_to_recieve = next_account_info(account_info_iter)?;

        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
        let token_program = next_account_info(account_info_iter)?;

        if !rent.is_exempt(staking_account.lamports(), staking_account.data_len()) {
            return Err(StakingError::NotRentExempt.into());
        }

        let mut staking_info = Staking::unpack_unchecked(&staking_account.data.borrow())?;
        
        if *initializer_xTokA_accnt_to_recieve.key!=staking_info.initializer_token_to_receive_account_pubkey{
            return Err(StakingError::NotExpectedTokenAccount.into());
        }

        if !staking_info.is_initialized{
            return Err(StakingError::NoMatchingDeposit.into());
        }
        

        if staking_info.amount_currently_locked<amount_to_mint{
            return Err(StakingError::MintAmountExceedsLockedValue.into());
        }


let xTokenA_mint_info = TokenAccount::unpack(&xTokA_mint.data.borrow())?;


msg!("TokenA_mint_info.owner: {} ", xTokenA_mint_info.owner);

let xtokenA_mint_ix = spl_token::instruction::mint_to(
    token_program.key,
    xTokA_mint.key,
    &initializer_xTokA_accnt_to_recieve.key,
    &initializer.key, 
    &[&xTokenA_mint_info.owner], 
    amount_to_mint
)?;
msg!("Calling the token program mint xTokenA to depositer...");

invoke(
    &xtokenA_mint_ix,
    &[
        xTokA_mint.clone(),
        initializer_xTokA_accnt_to_recieve.clone(),
        xTokA_minting_authority.clone(),
        token_program.clone(),
    ],
)?;

Ok(())
    }

}
