// The cfg_attr attribute conditionally includes attributes based on a configuration predicate.
// https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute
#![cfg_attr(not(feature = "std"), no_std)]

// https://github.com/paritytech/ink/blob/v4.0.0-beta.1/crates/ink/macro/src/contract.rs
// In a module annotated with #[ink::contract] these attributes are available...
// https://github.com/paritytech/ink
#[ink::contract]
mod az_light_switch {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    /// https://paritytech.github.io/ink/ink_ir/enum.ImplItem.html#variant.Constructor
    #[ink(storage)]
    pub struct LightSwitch {
        /// Stores a single `bool` value on the storage.
        on: bool,
    }

    impl LightSwitch {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { on: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
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
        pub fn get(&self) -> bool {
            self.on
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
            assert_eq!(az_light_switch.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut az_light_switch = LightSwitch::new(false);
            assert_eq!(az_light_switch.get(), false);
            az_light_switch.flip();
            assert_eq!(az_light_switch.get(), true);
        }
    }
}
