use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token};

declare_id!("D7PPVzQqSZmhYnsWRsLgiZ5qgmy2ZiDgEvqWWu91oJs7");

#[program]
pub mod mint_nft {
    use anchor_lang::{solana_program::{native_token::LAMPORTS_PER_SOL, program::invoke}, system_program};
    use anchor_spl::{associated_token, token};
    use mpl_token_metadata::{instructions::{CreateMasterEditionV3, CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs}, types::DataV2};

    use super::*;

    pub fn mint(
        ctx: Context<MintNFT>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String
    ) -> Result<()> {
        msg!("Creating mint account with the following key: {}", &ctx.accounts.mint.key());
        system_program::create_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.mint_authority.to_account_info(),
                    to: ctx.accounts.mint.to_account_info()
                }
            ),
            LAMPORTS_PER_SOL / 2,
            82,
            &ctx.accounts.token_program.key()
        )?;

        msg!("Initializing mint account");
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info()
                }
            ),
            0,
            &ctx.accounts.mint_authority.key(),
            Some(&ctx.accounts.mint_authority.key())
        )?;

        msg!("Creating token account with key {}", &ctx.accounts.token_account.key());
        associated_token::create(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                associated_token::Create {
                    mint: ctx.accounts.mint.to_account_info(),
                    payer: ctx.accounts.mint_authority.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                    associated_token: ctx.accounts.token_account.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info()
                }
            )
        )?;

        msg!("Minting token to account {}", &ctx.accounts.token_account.key());
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info()
                }
            ),
            1
        )?;

        msg!("Creating metadata account with key {}", &ctx.accounts.metadata.key());
        let create_metadata_account_args = CreateMetadataAccountV3InstructionArgs {
            collection_details: None,
            data: DataV2 {
                collection: None,
                creators: None,
                name: metadata_title,
                symbol: metadata_symbol,
                uri: metadata_uri,
                seller_fee_basis_points: 0,
                uses: None
            },
            is_mutable: false,
        };

        let create_metadata_account = CreateMetadataAccountV3 {
            metadata: ctx.accounts.metadata.key(),
            mint: ctx.accounts.mint.key(),
            payer: ctx.accounts.mint_authority.key(),
            mint_authority: ctx.accounts.mint_authority.key(),
            update_authority: (ctx.accounts.mint_authority.key(), true),
            rent: Some(ctx.accounts.rent.key()),
            system_program: ctx.accounts.system_program.key()
        };

        let create_metadata_account_ix = create_metadata_account.instruction(create_metadata_account_args);
        invoke(
            &create_metadata_account_ix,
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.rent.to_account_info()
            ]
        )?;

        msg!("Creating master edition metadata account with key {}", &ctx.accounts.master_edition.key());
        let create_master_edition_metadata_account_args = CreateMasterEditionV3InstructionArgs {
            max_supply: Some(0)
        };

        let create_master_edition_metadata_account = CreateMasterEditionV3 {
            edition: ctx.accounts.master_edition.key(),
            mint: ctx.accounts.mint.key(),
            payer: ctx.accounts.mint_authority.key(),
            mint_authority: ctx.accounts.mint_authority.key(),
            update_authority: ctx.accounts.mint_authority.key(),
            rent: Some(ctx.accounts.rent.key()),
            metadata: ctx.accounts.metadata.key(),
            system_program: ctx.accounts.system_program.key(),
            token_program: ctx.accounts.token_program.key()
        };

        let create_master_edition_metadata_account_ix = create_master_edition_metadata_account.instruction(create_master_edition_metadata_account_args);
        invoke(
            &create_master_edition_metadata_account_ix,
            &[
                ctx.accounts.master_edition.to_account_info(),
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.rent.to_account_info()
            ]
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint: Signer<'info>,

    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint_authority: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>
}
