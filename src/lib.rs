#![no_std]

// Import files here
pub mod config;

// Import Elrond Wasm
elrond_wasm::imports!();
// elrond_wasm::derive_imports!();

// Contract
#[elrond_wasm_derive::contract]
pub trait Pools:
 config::ConfigModule {
	#[init]
	fn init(
		&self,
		deposit_token: TokenIdentifier, // the deposit token generally it will be sARN
		reward_token: TokenIdentifier, // the reward token
		// _reward_amount: Self::BigUint,
		start_block: Self::BigUint,
		 end_block: Self::BigUint,
		// _fee_address: Address,
		#[var_args] opt_token_identifier: OptionalArg<TokenIdentifier>
	) -> SCResult<()> {

		Ok(())
	}

}