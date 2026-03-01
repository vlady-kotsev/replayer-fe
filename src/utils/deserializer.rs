#[cfg(feature = "ssr")]
pub mod deserializer {
    use serde::{Deserialize, Deserializer};
    use solana_keypair::Keypair;
    use solana_pubkey::Pubkey;

    pub fn deserialize_address<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        Ok(Pubkey::from_str_const(&s))
    }

    pub fn deserialize_keypair<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let keypair = Keypair::from_base58_string(&s);
        Ok(keypair.to_bytes())
    }
}
