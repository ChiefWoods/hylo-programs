use anchor_lang::prelude::*;

use super::*;

#[account]
#[derive(InitSpace)]
pub struct AddressUpdateProposal {
    pub address_field: AddressField,
    pub new_address: Pubkey,
    pub proposal_time: i64,
    pub ttl_secs: u64,
    pub approved: bool,
}

impl AddressUpdateProposal {
    pub fn init(
        &mut self,
        current_address: Pubkey,
        clock: &Clock,
        address_field: AddressField,
        new_address: Pubkey,
        ttl_secs: u64,
    ) -> Result<()> {
        require!(
            current_address != new_address,
            crate::error::ErrorCode::AdminNoop
        );
        let ttl_secs = Self::validate_ttl(ttl_secs)?;
        clock
            .unix_timestamp
            .checked_add(
                i64::try_from(ttl_secs)
                    .map_err(|_| crate::error::ErrorCode::AddressChangeTtlInvalid)?,
            )
            .ok_or(crate::error::ErrorCode::AddressChangeTtlInvalid)?;
        self.address_field = address_field;
        self.new_address = new_address;
        self.proposal_time = clock.unix_timestamp;
        self.ttl_secs = ttl_secs;
        self.approved = false;
        Ok(())
    }

    pub fn approve(&mut self, now: i64) -> Result<()> {
        self.require_live(now)?;
        require!(
            !self.approved,
            crate::error::ErrorCode::AddressChangeAlreadyApproved
        );
        self.approved = true;
        Ok(())
    }

    pub fn require_approved_and_live(&self, now: i64) -> Result<()> {
        self.require_live(now)?;
        require!(
            self.approved,
            crate::error::ErrorCode::AddressChangeNotApproved
        );
        Ok(())
    }

    pub fn require_live(&self, now: i64) -> Result<()> {
        let expires_at = self
            .proposal_time
            .checked_add(
                i64::try_from(self.ttl_secs)
                    .map_err(|_| crate::error::ErrorCode::AddressChangeExpired)?,
            )
            .ok_or(crate::error::ErrorCode::AddressChangeExpired)?;
        require!(
            now <= expires_at,
            crate::error::ErrorCode::AddressChangeExpired
        );
        Ok(())
    }

    fn validate_ttl(ttl_secs: u64) -> Result<u64> {
        require!(
            ttl_secs > 0 && i64::try_from(ttl_secs).is_ok(),
            crate::error::ErrorCode::AddressChangeTtlInvalid
        );
        Ok(ttl_secs)
    }
}
