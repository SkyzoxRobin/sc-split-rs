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
		#[var_args] args: VarArgs<MultiArg2<Address, Self::BigUint>> // recipients and amounts 
	) -> SCResult<()> {

		for payment in args.iter(){
			let recipient = payment.0.0.clone();
			let amount = payment.0.1.clone();		  
			self.send().direct_egld(&recipient, &amount, b"splitEGLD",); // should have recipient & amount as argument
		}
		Ok(())
	}


	// split esdt 

	// split sft

}