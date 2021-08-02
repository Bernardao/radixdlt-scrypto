use sbor::{Decode, Encode};

use crate::constructs::*;
use crate::kernel::*;
use crate::types::*;

/// An account holds tokens and badges.
#[derive(Debug, Clone, Encode, Decode)]
pub struct Account {
    address: Address,
}

impl From<Address> for Account {
    fn from(address: Address) -> Self {
        Self { address }
    }
}

impl Account {
    pub fn withdraw_tokens(&mut self, amount: U256, resource: &Resource) -> Tokens {
        let input = WithdrawTokensInput {
            account: self.address.clone(),
            amount,
            resource: resource.address(),
        };
        let output: WithdrawTokensOutput = call_kernel(WITHDRAW_TOKENS, input);

        output.tokens.into()
    }

    pub fn deposit_tokens(&mut self, tokens: Tokens) {
        let input = DepositTokensInput {
            account: self.address.clone(),
            tokens: tokens.into(),
        };
        let _: DepositTokensOutput = call_kernel(DEPOSIT_TOKENS, input);
    }

    pub fn withdraw_badges(&mut self, amount: U256, resource: &Resource) -> Badges {
        let input = WithdrawBadgesInput {
            account: self.address.clone(),
            amount,
            resource: resource.address(),
        };
        let output: WithdrawBadgesOutput = call_kernel(WITHDRAW_BADGES, input);

        output.badges.into()
    }

    pub fn deposit_badges(&mut self, badges: Badges) {
        let input = DepositBadgesInput {
            account: self.address.clone(),
            badges: badges.into(),
        };
        let _: DepositBadgesOutput = call_kernel(DEPOSIT_BADGES, input);
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }
}
