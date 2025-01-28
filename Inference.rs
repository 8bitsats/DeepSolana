use anchor_lang::prelude::*;

declare_id!("YOUR_PROGRAM_ID");

#[program]
pub mod model_manager {
    use super::*;

    // Initialize a model version
    pub fn initialize_model(ctx: Context<InitializeModel>, cid: String) -> Result<()> {
        ctx.accounts.model.cid = cid;
        Ok(())
    }

    // Update model CID after training
    pub fn update_model(ctx: Context<UpdateModel>, new_cid: String) -> Result<()> {
        ctx.accounts.model.cid = new_cid;
        Ok(())
    }
}

#[account]
pub struct ModelAccount {
    pub cid: String, // IPFS/Arweave CID
}
