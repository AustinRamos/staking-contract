use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct Staking {
    
    pub is_initialized: bool,
 
    pub vesting_period: u64,
    
    pub amount_currently_locked: u64,

    pub initializer_token_to_receive_account_pubkey: Pubkey,

}

impl Sealed for Staking {}

impl IsInitialized for Staking {
    fn is_initialized(&self) ->bool {
        self.is_initialized
    }
}

impl Pack for Staking {
    const LEN: usize = 49;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Staking::LEN];
        let  (
            is_initialized, 
            vesting_period,
            amount_currently_locked,
            initializer_token_to_receive_account_pubkey,
        )
        = array_refs![src,1,8,8,32];
        
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };


        Ok(Staking {
            is_initialized,
            vesting_period: u64::from_le_bytes(*vesting_period),
            amount_currently_locked: u64::from_le_bytes(*amount_currently_locked),
            initializer_token_to_receive_account_pubkey: Pubkey::new_from_array(*initializer_token_to_receive_account_pubkey),

        })
    }



    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Staking::LEN];

            let (is_initialized_dst,
                vesting_period_dst,
                amount_currently_locked_dst,
                initializer_token_to_receive_account_pubkey_dst,
            )  = mut_array_refs![dst, 1,8,8,32];

        let Staking {
            is_initialized,
            vesting_period,
            amount_currently_locked,
            initializer_token_to_receive_account_pubkey,
        } = self;
        is_initialized_dst[0] = *is_initialized as u8;
        *vesting_period_dst = vesting_period.to_le_bytes();
        *amount_currently_locked_dst = amount_currently_locked.to_le_bytes();
        initializer_token_to_receive_account_pubkey_dst.copy_from_slice(initializer_token_to_receive_account_pubkey.as_ref());
    }
}
