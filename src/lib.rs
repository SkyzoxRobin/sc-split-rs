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
	) -> SCResult<()> {
		Ok(())
	}

	#[payable("EGLD")]
	#[endpoint(splitEGLD)]
	fn split_egld(
		&self,
		#[payment_amount] token_amount: Self::BigUint,
		#[var_args] args: VarArgs<MultiArg2<Address, Self::BigUint>>
	) -> SCResult<()> {

		let mut sum = Self::BigUint::zero(); 
		let arguments = args.into_vec(); 

		for check_payment in arguments.clone() {
			let (_recipient, amount) = check_payment.into_tuple();
			sum += amount; 
		}

		require!(token_amount == sum, "The sum sent is not equal to the total amount to send");
		
		for payment in arguments {
			let (recipient, amount) = payment.into_tuple(); 
			self.send().direct_egld(&recipient, &amount, b"splitEGLD",);
		}

		Ok(())
	}

	// split esdt 
	#[payable("*")]
	#[endpoint(splitESDT)]
	fn split_esdt(
		&self,
		#[payment_token] token_id: TokenIdentifier,
		#[payment_amount] token_amount: Self::BigUint,
		#[var_args] args: VarArgs<MultiArg2<Address, Self::BigUint>>
	) -> SCResult<()> {

		let mut sum = Self::BigUint::zero();
		let arguments = args.into_vec(); 

		for payment in arguments.clone() {
			let (_recipient, amount) = payment.into_tuple(); 
			sum += amount; 
		}

		require!(token_amount == sum, "The sum sent is not equal to the total amount");

		for payment in arguments {
			let (recipient, amount) = payment.into_tuple();
			self.send().direct(&recipient, &token_id, 0, &amount, b"splitESDT",);
		}

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

		Ok(())
	}

}

splitEGLD {
    Sender: <votre adresse>
    Receiver: <l'adresse du smart contract>
    Value: <la valeur totale de ce que vous voulez envoyer>
    GasLimit: 20000000> // à augmenter si vous voulez envoyer vers beaucoup d'adresse
    Data: "splitEGLD" +
          "@" + <votre destinataire 1 > l'adresse du destinataire doit être décodé +
          "@" + <la valeur à envoyer à votre destinataire 1> +
          "@" + <votre destinataire 2 > l'adresse du destinataire doit être décodé +
          "@" + <la valeur à envoyer à votre destinataire 2> +
          "@" + <votre destinataire 3 > l'adresse du destinataire doit être décodé +
		  "@" + <la valeur à envoyer à votre destinataire 3> +
          <...>
}