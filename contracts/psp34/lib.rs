#![cfg_attr(not(feature = "std"), no_std, no_main)]
// #![feature(min_specialization)]

// pub use self::my_psp34::Mypsp34Ref;

#[openbrush::implementation(PSP34, Ownable, PSP34Mintable, PSP34Enumerable, PSP34Metadata)]
#[openbrush::contract]
pub mod my_psp34 {
    use openbrush::{
        modifiers,
        contracts::psp34::extensions::{
            enumerable::*,
            mintable::*,
        },
        contracts::ownable::*,
        traits::{Storage, String},
    };

    use ink::codegen::{EmitEvent, Env};

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
    	#[storage_field]
		psp34: psp34::Data,
		#[storage_field]
		ownable: ownable::Data,
        #[storage_field]
        enumerable: enumerable::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    #[overrider(PSP34Mintable)]
    #[modifiers(only_owner)]
    fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
        psp34::InternalImpl::_mint_to(self, account, id)
    }

    #[overrider(psp34::Internal)]
    fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
        self.env().emit_event(Transfer { from, to, id });
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
			ownable::Internal::_init_with_owner(&mut _instance, Self::env().caller());
            psp34::Internal::_mint_to(&mut _instance, Self::env().caller(), Id::U8(1))
                .expect("Can mint");
            let collection_id = psp34::PSP34Impl::collection_id(&_instance);
            metadata::Internal::_set_attribute(
                &mut _instance,
                collection_id.clone(),
                String::from("name"),
                String::from("Krb34"),
            );
            metadata::InternalImpl::_set_attribute(
                &mut _instance,
                collection_id,
                String::from("baseUri"),
                String::from("https://ipfs.io/ipfs/QmcHvJ5gPTEHSS8aNW9sTuxyJJynmoMb1j1FkKFRZHSNwy?filename=wizard.png")
            );
			_instance
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            let id_copy = id.clone();
            let mint_result = psp34::InternalImpl::_mint_to(self, openbrush::contracts::ownable::OwnableImpl::owner(self).unwrap(), id);
            // or easier 
            // let mint_result = psp34::InternalImpl::_mint_to(self, self.env().caller(), id);
            if mint_result.is_err() {
               return Err(PSP34Error::Custom(String::from("BadMint")));
            }  
            let transfer_result = psp34::InternalImpl::_transfer_token(self, account, id_copy, Vec::new());
            if transfer_result.is_err() {
                return Err(PSP34Error::Custom(String::from("BadTransfer")));
             }  
            Ok(())
        }
    }
}