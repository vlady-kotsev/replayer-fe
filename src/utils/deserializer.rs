#[cfg(feature = "ssr")]
pub mod deserializer {
    use serde::{de, Deserialize, Deserializer};
    use solana_address::Address;

    // pub fn deserialize_signature<'de, D>(deserializer: D) -> Result<Signature, D::Error>
    // where
    //     D: Deserializer<'de>,
    // {
    //     let s: String = Deserialize::deserialize(deserializer)?;
    //     let signature: Signature = s
    //         .parse()
    //         .map_err(|_| de::Error::custom("Failed to deserialize signature"))?;

    //     Ok(signature)
    // }

    pub fn deserialize_address<'de, D>(deserializer: D) -> Result<Address, D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded: String = Deserialize::deserialize(deserializer)?;
        let bytes: [u8; 32] = bs58::decode(encoded)
            .into_vec()
            .map_err(|_| de::Error::custom("Failed to deserialize address"))?
            .try_into()
            .map_err(|_| de::Error::custom("Invalid base58 string"))?;

        Ok(Address::new_from_array(bytes))
    }
}
