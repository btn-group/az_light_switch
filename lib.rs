// The cfg_attr attribute conditionally includes attributes based on a configuration predicate.
// https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute
#![cfg_attr(not(feature = "std"), no_std)]

// https://github.com/paritytech/ink/blob/v4.0.0-beta.1/crates/ink/macro/src/contract.rs
// In a module annotated with #[ink::contract] these attributes are available...
// https://github.com/paritytech/ink
#[ink::contract]
mod az_light_switch {
    use openbrush::{contracts::ownable::*, modifiers, traits::Storage};
    // === ENUMS ===
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum LightSwitchError {
        LightAlreadyOn,
        IncorrectFee(String),
    }

    // === STRUCTS ===
    #[derive(Debug, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Config {
        on: bool,
        on_fee: u128,
        off_payment: u128,
        admin: AccountId,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    /// https://paritytech.github.io/ink/ink_ir/enum.ImplItem.html#variant.Constructor
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct LightSwitch {
        on: bool,
        on_fee: u128,
        off_payment: u128,
        #[storage_field]
        ownable: ownable::Data,
    }

    impl LightSwitch {
        #[ink(constructor)]
        pub fn new(on_fee: u128, off_payment: u128) -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(Self::env().caller());
            instance.on_fee = on_fee;
            instance.off_payment = off_payment;
            instance
        }

        #[ink(message, payable)]
        pub fn turn_on(&mut self) -> Result<(), LightSwitchError> {
            if self.on {
                return Err(LightSwitchError::LightAlreadyOn);
            }
            if self.env().transferred_value() != self.on_fee {
                return Err(LightSwitchError::IncorrectFee(format!(
                    "Fee required: {}",
                    self.on_fee
                )));
            }

            self.on = true;
            Ok(())
        }

        #[ink(message)]
        pub fn config(&self) -> Config {
            Config {
                admin: self.ownable.owner(),
                on: self.on,
                on_fee: self.on_fee,
                off_payment: self.off_payment,
            }
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_config(
            &mut self,
            admin: Option<AccountId>,
            on_fee: Option<u128>,
            off_payment: Option<u128>,
        ) -> Result<(), OwnableError> {
            if admin.is_some() {
                self.ownable.transfer_ownership(admin.unwrap())?;
            }
            if on_fee.is_some() {
                self.on_fee = on_fee.unwrap();
            }
            if off_payment.is_some() {
                self.off_payment = off_payment.unwrap();
            }
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use openbrush::test_utils;

        // === HELPER FUNCTIONS ===
        fn get_balance(account_id: AccountId) -> Balance {
            ink::env::test::get_account_balance::<ink::env::DefaultEnvironment>(account_id)
                .expect("Cannot get account balance")
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(account_id, balance)
        }

        // === TESTS ===
        #[ink::test]
        fn test_turn_on() {
            let mut az_light_switch = LightSwitch::new(1, 1);
            // when light is already on
            // * it raises an error
            az_light_switch.on = true;
            let mut result = az_light_switch.turn_on();
            assert_eq!(result, Err(LightSwitchError::LightAlreadyOn));
            // when light is off
            az_light_switch.on = false;
            // = when wrong amount is sent in
            set_balance(az_light_switch.ownable.owner(), 10);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                az_light_switch.on_fee + 1,
            );
            result = az_light_switch.turn_on();
            assert_eq!(
                result,
                Err(LightSwitchError::IncorrectFee(format!(
                    "Fee required: {}",
                    az_light_switch.on_fee
                )))
            );
            // = when correct amount is sent in
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                az_light_switch.on_fee,
            );
            // = * is turns light on
            result = az_light_switch.turn_on();
            assert!(result.is_ok());
            assert_eq!(az_light_switch.on, true);
        }

        #[ink::test]
        fn test_update_config() {
            let accounts = test_utils::accounts();
            test_utils::change_caller(accounts.alice);
            let mut az_light_switch = LightSwitch::new(1, 1);
            // when called by a non-admin
            test_utils::change_caller(accounts.bob);
            // * it raises an error
            let mut result = az_light_switch.update_config(None, None, None);
            assert_eq!(result, Err(OwnableError::CallerIsNotOwner));
            // when called by an admin
            test_utils::change_caller(accounts.alice);
            result = az_light_switch.update_config(Some(accounts.django), Some(3), Some(4));
            assert!(result.is_ok());
            let config = az_light_switch.config();
            // * it updates the admin
            assert_eq!(config.admin, accounts.django);
            // * it updates the on_fee
            assert_eq!(config.on_fee, 3);
            // * it updates the off_payment
            assert_eq!(config.off_payment, 4)
        }
    }
}
