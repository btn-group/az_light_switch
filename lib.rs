// The cfg_attr attribute conditionally includes attributes based on a configuration predicate.
// https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute
#![cfg_attr(not(feature = "std"), no_std)]

// https://github.com/paritytech/ink/blob/v4.0.0-beta.1/crates/ink/macro/src/contract.rs
// In a module annotated with #[ink::contract] these attributes are available...
// https://github.com/paritytech/ink
#[ink::contract]
mod az_light_switch {
    use openbrush::{contracts::ownable::*, modifiers, traits::Storage};

    #[derive(Debug, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Config {
        on: bool,
        on_fee: u32,
        off_payment: u32,
        admin: AccountId,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    /// https://paritytech.github.io/ink/ink_ir/enum.ImplItem.html#variant.Constructor
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct LightSwitch {
        /// Stores config in storage
        on: bool,
        on_fee: u32,
        off_payment: u32,
        #[storage_field]
        ownable: ownable::Data,
    }

    impl LightSwitch {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(on_fee: u32, off_payment: u32) -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(Self::env().caller());
            instance.on_fee = on_fee;
            instance.off_payment = off_payment;
            instance
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.on = !self.on;
        }

        /// Simply returns the current value of our `bool`.
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
            on_fee: Option<u32>,
            off_payment: Option<u32>,
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
        use ink::env::{test, DefaultEnvironment};

        #[ink::test]
        fn test_update_config() {
            let accounts = test::default_accounts::<DefaultEnvironment>();
            test::set_caller::<DefaultEnvironment>(accounts.alice);
            let mut az_light_switch = LightSwitch::new(1, 1);
            // when called by a non-admin
            test::set_caller::<DefaultEnvironment>(accounts.bob);
            // * it raises an error
            let mut result = az_light_switch.update_config(None, None, None);
            assert_eq!(result, Err(OwnableError::CallerIsNotOwner));
            // when called by an admin
            test::set_caller::<DefaultEnvironment>(accounts.alice);
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
