// The cfg_attr attribute conditionally includes attributes based on a configuration predicate.
// https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute
#![cfg_attr(not(feature = "std"), no_std)]

// https://github.com/paritytech/ink/blob/v4.0.0-beta.1/crates/ink/macro/src/contract.rs
// In a module annotated with #[ink::contract] these attributes are available...
// https://github.com/paritytech/ink
#[ink::contract]
mod az_light_switch {
    #[derive(Debug, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Config {
        on: bool,
        on_fee: u32,
        off_payment: u32,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    /// https://paritytech.github.io/ink/ink_ir/enum.ImplItem.html#variant.Constructor
    #[ink(storage)]
    pub struct LightSwitch {
        /// Stores config in storage
        on: bool,
        on_fee: u32,
        off_payment: u32,
    }

    impl LightSwitch {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(on_fee: u32, off_payment: u32) -> Self {
            Self {
                on: false,
                on_fee,
                off_payment,
            }
        }

        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(2, 1)
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
                on: self.on,
                on_fee: self.on_fee,
                off_payment: self.off_payment,
            }
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let az_light_switch = LightSwitch::default();
            let config: Config = az_light_switch.config();
            assert_eq!(config.on, false);
            assert_eq!(config.on_fee, 2);
            assert_eq!(config.off_payment, 1);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let az_light_switch = LightSwitch::default();
            let config: Config = az_light_switch.config();
            assert_eq!(config.on, false);
            // az_light_switch.flip();
            // assert_eq!(az_light_switch.get(), true);
        }
    }
}
