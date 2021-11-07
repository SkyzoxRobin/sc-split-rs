#![no_std]

// Import files here
pub mod config;

// Import Elrond Wasm
elrond_wasm::imports!();
// elrond_wasm::derive_imports!();

#[elrond_wasm_derive::contract]
pub trait Presale:
 config::ConfigModule 
 {
	#[init]
	fn init(
		&self,
		the_nft_price: BigUint,
	) -> SCResult<()> {
		self.nft_price().set(&the_nft_price);
		Ok(())
	}

	// fn issue token
	#[only_owner]
	#[payable("EGLD")]
	#[endpoint(issuePresaleToken)]
	fn issue_presale_token(
		&self,
		#[payment] issue_cost: BigUint,
        token_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
	) -> SCResult<AsyncCall> {
		require!(self.presale_token_id().is_empty(), "Already issued");

        Ok(self.send()
            .esdt_system_sc_proxy()
            .issue_semi_fungible(
                issue_cost,
                &token_name,
                &token_ticker,
                SemiFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(
                self.callbacks()
                    .issue_callback(&self.blockchain().get_caller()))
            )
    }

	#[only_owner]
	#[endpoint(setLocalRolesPresaleToken)]
	#[payable("EGLD")]
	fn set_local_roles_presale_token(&self) -> SCResult<AsyncCall> {
		// self.require_token_issued()?;
		require!(!self.presale_token_id().is_empty(), "No presale token issued");

		let token = self.presale_token_id().get();

		let roles = [
            EsdtLocalRole::NftCreate,
            EsdtLocalRole::NftAddQuantity,
            EsdtLocalRole::NftBurn,
        ];

		Ok(self.send()
			.esdt_system_sc_proxy()
			.set_special_roles(
				&self.blockchain().get_sc_address(),
				&token,
				(&roles[..]).into_iter().cloned()
			)
			.async_call())

	}


	// fn enter presale

	#[payable("EGLD")]
	#[endpoint(enterPresale)]
	fn enter_presale(
		&self,
		#[payment] _payment: BigUint,
		// #[var_args] args: VarArgs<MultiArg2<ManagedAddress, BigUint>>
	) -> SCResult<()> {
		require!(!self.presale_token_id().is_empty(), "No presale token issued");

			// get caller 
		let _caller = self.blockchain().get_caller();
		let _token = self.presale_token_id().get();
			// add quantity sft
			
			// send SFT to the caller
		Ok(())
	}

	// fn enter presale only for whitelisted people

	//view 

	#[view(getNftPrice)]
    #[storage_mapper("getNftPrice")]
    fn nft_price(&self) -> SingleValueMapper<BigUint>;

	// whitelist 

	// token id

	#[view(nftTokenId)]
    #[storage_mapper("nftTokenId")]
    fn presale_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

	//callback
	#[callback]
    fn issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {

                if self.presale_token_id().is_empty() {
                    self.presale_token_id().set(&token_id);
                }
            }
            ManagedAsyncCallResult::Err(_) => {

                let (returned_tokens, token_id) = self.call_value().payment_token_pair();
                if token_id.is_egld() && returned_tokens > 0 {
                    let _ = self.send().direct_egld(caller, &returned_tokens, &[]);
                }
            }
        }
    }

}