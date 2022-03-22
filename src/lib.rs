#![no_std]

elrond_wasm::imports!();

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
		#[var_args] args: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>
	) {
		let mut sum = BigUint::zero(); 

		for check_payment in args.into_iter() {
			let (recipient, amount) = check_payment.into_tuple();
			sum += amount.clone();

			self.send().direct_egld(&recipient, &amount, &[],);
		}
		require!(token_amount == sum, "The sum sent is not equal to the total amount to send");
	}

	#[payable("*")]
	#[endpoint(splitESDT)]
	fn split_esdt(
		&self,
		#[payment_token] token_id: TokenIdentifier,
		#[payment_amount] token_amount: BigUint,
		#[var_args] args: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>
	) {
		let mut sum = BigUint::zero();

		for check_payment in args.into_iter() {
			let (recipient, amount) = check_payment.into_tuple(); 
			sum += amount.clone(); 

			self.send().direct(&recipient, &token_id, 0, &amount, &[],);
		}
		require!(token_amount == sum, "The sum sent is not equal to the total amount");
	}
}