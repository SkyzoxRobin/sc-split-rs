elrond_wasm::imports!();
// elrond_wasm::derive_imports!();

#[elrond_wasm_derive::module]
pub trait ConfigModule: {
    
    #[view(getDurationInBlocks)]
    #[storage_mapper("duration_in_blocks")]
    fn duration_in_blocks(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;
    
    #[view(getDurationInSeconds)]
    #[storage_mapper("duration_in_seconds")]
    fn duration_in_seconds(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

}