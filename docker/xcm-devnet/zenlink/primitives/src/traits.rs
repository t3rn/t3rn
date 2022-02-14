/// Extension trait for CurrencyId
pub trait CurrencyIdExt {
	type TokenSymbol;
	fn is_vtoken(&self) -> bool;
	fn is_token(&self) -> bool;
	fn is_vstoken(&self) -> bool;
	fn is_vsbond(&self) -> bool;
	fn is_native(&self) -> bool;
	fn is_stable(&self) -> bool;
	fn is_lptoken(&self) -> bool;
	fn into(symbol: Self::TokenSymbol) -> Self;
}