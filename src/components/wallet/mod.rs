#[allow(clippy::module_inception)]
pub mod wallet;
pub use wallet::Wallet; // ← re-exporta para o nível acima
