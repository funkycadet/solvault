use anchor_lang::prelude::*;

declare_id!("5XuG1vnwQnq25fYz13KFiYc86boHM7KihY7m4GmiVGo4");

#[program]
mod solvault {
    use super::*;

    pub fn mint_nft(ctx: Context<MintNFT>, metadata: String, uri: String) -> Result<()> {
        let nft = &mut ctx.accounts.nft;
        nft.owner = *ctx.accounts.owner.key;
        nft.metadata = metadata;
        nft.uri = uri;
        Ok(())
    }

    pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner = *ctx.accounts.owner.key;
        Ok(())
    }

    pub fn lock_nft(ctx: Context<LockNFT>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let nft = &mut ctx.accounts.nft;

        // require!(vault.owner == *ctx.accounts.owner.key, CustomError::Unauthorized);
        nft.owner = vault.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 256 + 256)]
    pub nft: Account<'info, NFT>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(init, payer = owner, space = 8 + 32)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LockNFT<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub nft: Account<'info, NFT>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
}

#[account]
pub struct NFT {
    pub owner: Pubkey,
    pub metadata: String,
    pub uri: String,
}

// #[error]
// pub enum CustomError {
//     #[msg("Unauthorized")]
//     Unauthorized,
// }
