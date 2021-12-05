#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait Disperse
 {
	#[init]
	fn init(
		&self,
	) {}

	#[payable("EGLD")]
	#[endpoint(splitEGLD)]
	fn split_egld(
		&self,
		#[payment_amount] token_amount: BigUint,
		#[var_args] args: VarArgs<MultiArg2<ManagedAddress, BigUint>>
	) -> SCResult<()> {

		let mut sum = BigUint::zero(); 
		let arguments = args.into_vec(); 

		for check_payment in arguments.clone() {
			let (_recipient, amount) = check_payment.into_tuple();
			sum += amount; 
		}

		require!(token_amount == sum, "The sum sent is not equal to the total amount to send");
		
		for split_payment in arguments {
			let (recipient, amount) = split_payment.into_tuple(); 
			self.send().direct_egld(&recipient, &amount, &[],);
		}
		Ok(())
	}

	// split esdt 
	#[payable("*")]
	#[endpoint(splitESDT)]
	fn split_esdt(
		&self,
		#[payment_token] token_id: TokenIdentifier,
		#[payment_amount] token_amount: BigUint,
		#[var_args] args: VarArgs<MultiArg2<ManagedAddress, BigUint>>
	) -> SCResult<()> {

		let mut sum = BigUint::zero();
		let arguments = args.into_vec(); 

		for check_payment in arguments.clone() {
			let (_recipient, amount) = check_payment.into_tuple(); 
			sum += amount; 
		}

		require!(token_amount == sum, "The sum sent is not equal to the total amount");

		for split_payment in arguments {
			let (recipient, amount) = split_payment.into_tuple();
			self.send().direct(&recipient, &token_id, 0, &amount, b"splitESDT",);
		}
		Ok(())
	}

	// split sft
	// #[payable("*")]
	// #[endpoint(splitSFT)]
	// fn split_sft(
	//	&self,
	//	#[var_args] args: VarArgs<MultiArg4<TokenIdentifier, u64, Address, Self::BigUint>>
	// ) -> SCResult<()> {

	//	for payment in args.into_vec(){
	//		let (token_id, nonce, recipient, amount) = payment.into_tuple();
	//		self.send().direct(&recipient, &token_id, nonce, &amount, b"splitESDT",);
	//	}

	//	Ok(())
	//}

}