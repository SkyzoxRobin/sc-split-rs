#![no_std]

// Import files here
pub mod config;

// Import Elrond Wasm
elrond_wasm::imports!();
// elrond_wasm::derive_imports!();

// Contract
#[elrond_wasm_derive::contract]
pub trait Pools:
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
		// recipients: Vec<Address>,
		// amount: Vec<Self::BigUint>
		#[var_args] VarArgs<MultiArg2<Address, BigUint>>
	) -> SCResult<()> {

		for i in 0..recipients.len() {
			self.send().direct_egld(&Address[i], &amount[i], b"splitEGLD",);



			// get caller 
			// let _caller = self.blockchain().get_caller();
			// get balance of the caller
		}
		Ok(())
	}

	  // contract Disperse {
		 //	function disperseEther(address[] recipients, uint256[] values) external payable {
		//	for (uint256 i = 0; i < recipients.length; i++)
			//	recipients[i].transfer(values[i]);
		//	uint256 balance = address(this).balance;
		//	if (balance > 0)
		//		msg.sender.transfer(balance);
		//	}

	// disperse esdt 

}