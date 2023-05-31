// The cfg_attr attribute conditionally includes attributes based on a configuration predicate.
// https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute
#![cfg_attr(not(feature = "std"), no_std)]

// https://github.com/paritytech/ink/blob/v4.0.0-beta.1/crates/ink/macro/src/contract.rs
// In a module annotated with #[ink::contract] these attributes are available...
// https://github.com/paritytech/ink
#[ink::contract]
mod az_light_switch {
    use ink::prelude::{vec, vec::Vec};
    use ink::storage::Mapping;
    use openbrush::{contracts::ownable::*, modifiers, traits::Storage};
    // === ENUMS ===
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum LightSwitchError {
        LightAlreadyOff,
        LightAlreadyOn,
        IncorrectFee,
        InsufficientBalance,
        InsufficientTimePassed,
        RecordsLimitReached,
    }

    // === STRUCTS ===
    #[derive(Debug, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Config {
        on: bool,
        minimum_on_time_in_ms: Timestamp,
        on_time: Option<Timestamp>,
        on_fee: Balance,
        off_payment: Balance,
        admin: AccountId,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug, Clone)]
    pub struct Record {
        id: u32,
        caller: AccountId,
        on: bool,
        block_height: BlockNumber,
    }

    #[derive(Debug, Default)]
    #[ink::storage_item]
    pub struct Records {
        values: Mapping<u32, Record>,
        length: u32,
    }
    impl Records {
        pub fn index(&self, page: u32, size: u8) -> Vec<Record> {
            let mut records: Vec<Record> = vec![];
            if self.length == 0 || size == 0 {
                return records;
            }

            let records_to_skip_wrapped: Option<u32> = page.checked_mul(size.into());
            let ending_index: u32;
            let starting_index: u32;
            // If there's two items 0 and 1
            // If we're skipping zero items we want index 1..=0
            // when skipping 1 item, we want index 0..=0
            // If records_to_skip is greater than the length, return empty
            if let Some(records_to_skip) = records_to_skip_wrapped {
                if records_to_skip >= self.length {
                    return records;
                }

                ending_index = (self.length - 1).saturating_sub(records_to_skip);
                starting_index = ending_index.saturating_sub(size.into());
            } else {
                return records;
            }
            for i in (starting_index..=ending_index).rev() {
                records.push(self.values.get(i).unwrap())
            }
            records
        }

        pub fn create(&mut self, value: &Record) {
            if self.values.insert(self.length, value).is_none() {
                self.length += 1
            }
        }
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct RecordsForFrontEnd {
        values: Vec<Record>,
        length: u32,
    }

    // Events
    // Many of these fields wouldn't be necessary for real use
    // but they are here so that we can look at them in the block explorer
    #[ink(event)]
    pub struct TurnOn {
        #[ink(topic)]
        admin: AccountId,
        #[ink(topic)]
        caller: AccountId,
        on: bool,
        value: Balance,
    }

    // Trialling anonymous with this one
    #[ink(event)]
    #[ink(anonymous)]
    pub struct TurnOff {
        #[ink(topic)]
        admin: AccountId,
        #[ink(topic)]
        caller: AccountId,
        on: bool,
        value: Balance,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    /// https://paritytech.github.io/ink/ink_ir/enum.ImplItem.html#variant.Constructor
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct LightSwitch {
        on: bool,
        minimum_on_time_in_ms: Timestamp,
        on_time: Option<Timestamp>,
        on_fee: Balance,
        off_payment: Balance,
        #[storage_field]
        ownable: ownable::Data,
        records: Records,
    }

    impl LightSwitch {
        #[ink(constructor)]
        pub fn new(
            on_fee: Balance,
            off_payment: Balance,
            minimum_on_time_in_ms: Timestamp,
        ) -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(Self::env().caller());
            instance.on_fee = on_fee;
            instance.off_payment = off_payment;
            instance.minimum_on_time_in_ms = minimum_on_time_in_ms;
            instance.records = Records {
                values: Mapping::default(),
                length: 0,
            };
            instance
        }

        #[ink(message, payable)]
        pub fn turn_on(&mut self) -> Result<(), LightSwitchError> {
            if self.on {
                return Err(LightSwitchError::LightAlreadyOn);
            }
            if self.env().transferred_value() != self.on_fee {
                return Err(LightSwitchError::IncorrectFee);
            }
            if self.records.length == u32::MAX {
                return Err(LightSwitchError::RecordsLimitReached);
            }

            self.on_time = Some(self.env().block_timestamp());
            self.on = true;

            // emit event
            self.env().emit_event(TurnOn {
                admin: self.ownable.owner(),
                caller: Self::env().caller(),
                on: self.on,
                value: self.on_fee,
            });

            // store record
            self.records.create(&Record {
                id: self.records.length,
                caller: Self::env().caller(),
                on: self.on,
                block_height: self.env().block_number(),
            });

            Ok(())
        }

        #[ink(message, payable)]
        pub fn turn_off(&mut self) -> Result<(), LightSwitchError> {
            if !self.on {
                return Err(LightSwitchError::LightAlreadyOff);
            }
            if self.env().balance() < self.off_payment {
                return Err(LightSwitchError::InsufficientBalance);
            }
            if self.env().block_timestamp() < self.on_time.unwrap() + self.minimum_on_time_in_ms {
                return Err(LightSwitchError::InsufficientTimePassed);
            }
            if self.records.length == u32::MAX {
                return Err(LightSwitchError::RecordsLimitReached);
            }

            if self
                .env()
                .transfer(self.env().caller(), self.off_payment)
                .is_err()
            {
                panic!(
                    "requested transfer failed. this can be the case if the contract does not\
                     have sufficient free funds or if the transfer would have brought the\
                     contract's balance below minimum balance."
                )
            }

            self.on_time = None;
            self.on = false;

            // emit event
            self.env().emit_event(TurnOff {
                admin: self.ownable.owner(),
                caller: Self::env().caller(),
                on: self.on,
                value: self.off_payment,
            });

            // store record
            self.records.create(&Record {
                id: self.records.length,
                caller: Self::env().caller(),
                on: self.on,
                block_height: self.env().block_number(),
            });

            Ok(())
        }

        #[ink(message)]
        pub fn config(&self) -> Config {
            Config {
                admin: self.ownable.owner(),
                on: self.on,
                minimum_on_time_in_ms: self.minimum_on_time_in_ms,
                on_time: self.on_time,
                on_fee: self.on_fee,
                off_payment: self.off_payment,
            }
        }

        #[ink(message)]
        pub fn records(&self, page: u32, size: u8) -> RecordsForFrontEnd {
            RecordsForFrontEnd {
                values: self.records.index(page, size),
                length: self.records.length,
            }
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_config(
            &mut self,
            admin: Option<AccountId>,
            on_fee: Option<Balance>,
            off_payment: Option<Balance>,
            minimum_on_time_in_ms: Option<Timestamp>,
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
            if minimum_on_time_in_ms.is_some() {
                self.minimum_on_time_in_ms = minimum_on_time_in_ms.unwrap();
            }
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use openbrush::test_utils;

        // === HELPER FUNCTIONS ===
        fn contract_id() -> AccountId {
            ink::env::test::callee::<ink::env::DefaultEnvironment>()
        }

        fn get_balance(account_id: AccountId) -> Balance {
            ink::env::test::get_account_balance::<ink::env::DefaultEnvironment>(account_id)
                .expect("Cannot get account balance")
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(account_id, balance)
        }

        fn get_current_time() -> Timestamp {
            let since_the_epoch = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards");
            since_the_epoch.as_millis().try_into().unwrap()
        }

        // === TESTS ===
        #[ink::test]
        fn test_records() {
            let _accounts = test_utils::accounts();
            let mut az_light_switch = LightSwitch::new(1, 1, 1);
            // when records do not exist
            let mut result = az_light_switch.records(0, 0);
            // * it returns empty values
            assert_eq!(result.values.len(), 0);
            // * it returns a length of 0
            assert_eq!(result.length, 0);

            // when records exist
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                az_light_switch.on_fee,
            );
            let _ = az_light_switch.turn_on();
            // = when page is 0 and size is 0
            result = az_light_switch.records(0, 0);
            // == when page is 0 and size is 0
            // == * it returns an empty array
            assert_eq!(result.values.len(), 0);
            assert_eq!(result.length, 1);
            // == when page is 0 and size is 1
            result = az_light_switch.records(0, 1);
            // == * it return the record and a length of 1
            assert_eq!(result.values.len(), 1);
            assert_eq!(result.length, 1);
            // == when page is 1 and size is 1
            result = az_light_switch.records(1, 1);
            // == * it returns an empty array
            assert_eq!(result.values.len(), 0);
            assert_eq!(result.length, 1);
        }

        #[ink::test]
        fn test_turn_off() {
            let accounts = test_utils::accounts();
            let mut az_light_switch = LightSwitch::new(1, 1, 1);
            // when light is already off
            // * it raises an error
            let mut result = az_light_switch.turn_off();
            assert_eq!(result, Err(LightSwitchError::LightAlreadyOff));
            // when light is on
            az_light_switch.on = true;
            // = when contract balance in less than off_payment
            set_balance(contract_id(), 0);
            // = * it raises an error
            result = az_light_switch.turn_off();
            assert_eq!(result, Err(LightSwitchError::InsufficientBalance));
            // = when contract balance in equal to or greater than off_payment
            set_balance(contract_id(), az_light_switch.off_payment);
            test_utils::change_caller(accounts.bob);
            set_balance(accounts.bob, 0);
            // == when minimum_on_time_in_ms has not passed
            let current_time: Timestamp = get_current_time();
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(current_time);
            az_light_switch.on_time = Some(current_time);
            // == * it raises and error
            result = az_light_switch.turn_off();
            assert_eq!(result, Err(LightSwitchError::InsufficientTimePassed));
            // == when minimum_on_time_in_ms has passed
            az_light_switch.on_time = Some(current_time - 1);
            // == * is turns light off
            result = az_light_switch.turn_off();
            assert!(result.is_ok());
            assert_eq!(az_light_switch.on, false);
            // == * it sends the off_payment to the caller
            assert_eq!(get_balance(accounts.bob), az_light_switch.off_payment);
            // == * it sets the on_time to None
            assert_eq!(az_light_switch.on_time, None);
        }

        #[ink::test]
        fn test_turn_on() {
            let mut az_light_switch = LightSwitch::new(1, 1, 1);
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
            assert_eq!(result, Err(LightSwitchError::IncorrectFee));
            // = when correct amount is sent in
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                az_light_switch.on_fee,
            );
            let current_time: Timestamp = get_current_time();
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(current_time);
            result = az_light_switch.turn_on();
            assert!(result.is_ok());
            // = * is turns light on
            assert_eq!(az_light_switch.on, true);
            // = * it sets the on_time
            assert_eq!(az_light_switch.on_time, Some(current_time));
        }

        #[ink::test]
        fn test_update_config() {
            let accounts = test_utils::accounts();
            test_utils::change_caller(accounts.alice);
            let mut az_light_switch = LightSwitch::new(1, 1, 1);
            // when called by a non-admin
            test_utils::change_caller(accounts.bob);
            // * it raises an error
            let mut result = az_light_switch.update_config(None, None, None, None);
            assert_eq!(result, Err(OwnableError::CallerIsNotOwner));
            // when called by an admin
            test_utils::change_caller(accounts.alice);
            result =
                az_light_switch.update_config(Some(accounts.django), Some(3), Some(4), Some(5));
            assert!(result.is_ok());
            let config = az_light_switch.config();
            // * it updates the admin
            assert_eq!(config.admin, accounts.django);
            // * it updates the on_fee
            assert_eq!(config.on_fee, 3);
            // * it updates the off_payment
            assert_eq!(config.off_payment, 4);
            // * it updates the minimum_on_time_in_ms
            assert_eq!(config.minimum_on_time_in_ms, 5)
        }
    }
}
