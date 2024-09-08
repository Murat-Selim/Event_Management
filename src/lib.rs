use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("Bv5tPuhh455m3N12Dg5Sxf5GQxCqiHWcBqWfEVR9443e");

#[program]
pub mod event_management {
    use super::*;

    pub fn create_event(
        ctx: Context<CreateEvent>,
        name: String,
        description: String,
        date: String,
        max_participants: u32,
        price: u64,
    ) -> Result<()> {
        let event = &mut ctx.accounts.event;
        let organizer = &ctx.accounts.organizer;

        event.organizer = organizer.key();
        event.name = name;
        event.description = description;
        event.date = date;
        event.max_participants = max_participants;
        event.participants = Vec::new();
        event.price = price;
        event.token_mint = ctx.accounts.token_mint.key();

        Ok(())
    }

    pub fn join_event(ctx: Context<JoinEvent>) -> Result<()> {
        let event = &mut ctx.accounts.event;
        let participant = &ctx.accounts.participant;

        require!(
            event.participants.len() < event.max_participants as usize,
            EventError::EventFull
        );

        require!(
            !event.participants.contains(&participant.key()),
            EventError::AlreadyJoined
        );

        // Transfer tokens from participant to organizer
        let transfer_instruction = Transfer {
            from: ctx.accounts.participant_token_account.to_account_info(),
            to: ctx.accounts.event_token_account.to_account_info(),
            authority: participant.to_account_info(),
        };

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                transfer_instruction,
            ),
            event.price,
        )?;

        event.participants.push(participant.key());

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(mut)]
    pub organizer: Signer<'info>,
    #[account(
        init,
        payer = organizer,
        space = 8 + 32 + 64 + 256 + 8 + 4 + (32 * 50) + 8 + 32,
        seeds = [b"event", organizer.key().as_ref()],
        bump
    )]
    pub event: Account<'info, Event>,
    #[account(
        init,
        payer=organizer,
        associated_token::mint = token_mint,
        associated_token::authority = event
    )]
    pub event_token_account:Account<'info,TokenAccount>,
    #[account(mut)]
    pub token_mint: Account<'info, token::Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct JoinEvent<'info> {
    #[account(
        mut,    
        seeds = [b"event", participant.key().as_ref()],
        bump
        )]
    pub event: Account<'info, Event>,
     #[account(
        mut,
        associated_token::mint = event.token_mint,
        associated_token::authority = event
    )]
    pub event_token_account:Account<'info,TokenAccount>,
    #[account(mut)]
    pub participant: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = event.token_mint,
        associated_token::authority = participant
    )]
    pub participant_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Event {
    pub organizer: Pubkey,
    pub name: String,
    pub description: String,
    pub date: String,
    pub max_participants: u32,
    pub participants: Vec<Pubkey>,
    pub price: u64,
    pub token_mint: Pubkey,
    pub bump:u8
}

#[error_code]
pub enum EventError {
    #[msg("Event is already full")]
    EventFull,
    #[msg("You have already joined this event")]
    AlreadyJoined,
}
