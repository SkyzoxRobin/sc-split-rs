#![no_std]

// Import files here
pub mod config;

// Import Elrond Wasm
elrond_wasm::imports!();
// elrond_wasm::derive_imports!();

#[elrond_wasm_derive::contract]
pub trait Disperse:
 config::ConfigModule 
 {
	#[init]
	fn init(
		&self,
		// token_id = TokenIdentifier, 
	) -> SCResult<()> {
		Ok(())
	}

	#[payable("EGLD")]
	#[endpoint(splitEGLD)]
	fn split_egld(
		&self,
		#[var_args] args: VarArgs<MultiArg2<Address, Self::BigUint>>
	) -> SCResult<()> {

		for payment in args.iter(){
			let recipient = payment.0.0.clone();
			let amount = payment.0.1.clone();		  
			self.send().direct_egld(&recipient, &amount, b"splitEGLD",);

			//  get caller 
			// 	returned the excess amount or return an error if the total amount is not right
			// get total amount to send 
		}
		Ok(())
	}


	// split esdt 
	#[payable("*")]
	#[endpoint(splitESDT)]
	fn split_esdt(
		&self,
		#[var_args] args: VarArgs<MultiArg3<TokenIdentifier, Address, Self::BigUint>>
	) -> SCResult<()> {

		for payment in args.into_vec(){
			let (token_id, recipient, amount) = payment.into_tuple();
			self.send().direct(&recipient, &token_id, 0, &amount, b"splitESDT",);
		}

		// get caller 
		// returned the excess amount or return an error if the total amout sent is not right
		// get total amount to send
		Ok(())
	}

	// split sft
	#[payable("*")]
	#[endpoint(splitSFT)]
	fn split_sft(
		&self,
		#[var_args] args: VarArgs<MultiArg4<TokenIdentifier, u64, Address, Self::BigUint>>
	) -> SCResult<()> {

		for payment in args.into_vec(){
			let (token_id, nonce, recipient, amount) = payment.into_tuple();
			self.send().direct(&recipient, &token_id, nonce, &amount, b"splitESDT",);
		}

		// get caller 
		// returned the excess amount or return an error if the total amout sent is not right
		// get total amount to send
		Ok(())
	}



}