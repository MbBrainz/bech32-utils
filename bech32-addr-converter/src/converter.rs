use bech32::Bech32;

#[derive(std::fmt::Debug)]
pub enum Bech32Error {
    EncodeError(bech32::EncodeError),
    DecodeError(bech32::DecodeError),
    HrpError(bech32::primitives::hrp::Error),
}

impl From<bech32::EncodeError> for Bech32Error {
    fn from(error: bech32::EncodeError) -> Self {
        Bech32Error::EncodeError(error)
    }
}

impl From<bech32::DecodeError> for Bech32Error {
    fn from(error: bech32::DecodeError) -> Self {
        Bech32Error::DecodeError(error)
    }
}

impl From<bech32::primitives::hrp::Error> for Bech32Error {
    fn from(error: bech32::primitives::hrp::Error) -> Self {
        Bech32Error::HrpError(error)
    }
}
/// This function transfer the addr to a local neutron addr

pub fn any_addr_to_prefix_addr(addr: String, prefix: &str) -> Result<String, Bech32Error> {
    let (_hrp, data) = bech32::decode(&addr)?;
    let address = canonical_addr_to_prefix_addr(data, prefix)?;
    Ok(address)
}

pub fn canonical_addr_to_prefix_addr(
    canonical_addr: impl Into<Vec<u8>>,
    prefix: &str,
) -> Result<String, Bech32Error> {
    Ok(bech32::encode::<Bech32>(
        bech32::Hrp::parse(prefix)?,
        &canonical_addr.into(),
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;

    // const JUNO_BECH32_PREFIX: &str = "juno";
    const NEUTRON_BECH32_PREFIX: &str = "neutron";
    const OSMOSIS_BECH32_PREFIX: &str = "osmo";
    const TERRA_BECH32_PREFIX: &str = "terra";
    const KUJIRA_BECH32_PREFIX: &str = "kujira";

    #[test]
    fn test_any_addr_to_neutron() -> Result<(), Bech32Error> {
        let juno_addr_dan = "juno14wd9r77c2rlau7r8lta6mh8nqsvqq00rxha3fl";
        let juno_addr_eliseo = "juno1e7faq9nlpms0hfddd8vqqgval478shukpj8c50";
        let cosmos_addr_alecp = "cosmos1e4qv3j8lr6xuxsd9fy9yr9xx4hdcwnp4usgg4w";

        let neutron_addr_alecp =
            any_addr_to_prefix_addr(cosmos_addr_alecp.to_string(), NEUTRON_BECH32_PREFIX)?;
        println!("neutron_addr_alecp: {}", neutron_addr_alecp);

        // test juno to neutron and print the result
        let juno_addr_dan_neutron =
            any_addr_to_prefix_addr(juno_addr_dan.to_string(), NEUTRON_BECH32_PREFIX)?;
        let juno_addr_eliseo_neutron =
            any_addr_to_prefix_addr(juno_addr_eliseo.to_string(), NEUTRON_BECH32_PREFIX)?;
        println!("juno_addr_dan_neutron: \t{}", juno_addr_dan_neutron);
        println!("juno_addr_eliseo_neutron: {}", juno_addr_eliseo_neutron);
        print!("\n");

        // test juno to osmosis and print the result
        let juno_addr_dan_osmosis =
            any_addr_to_prefix_addr(juno_addr_dan.to_string(), OSMOSIS_BECH32_PREFIX)?;
        let juno_addr_eliseo_osmosis =
            any_addr_to_prefix_addr(juno_addr_eliseo.to_string(), OSMOSIS_BECH32_PREFIX)?;
        println!("juno_addr_dan_osmosis:\t {}", juno_addr_dan_osmosis);
        println!("juno_addr_eliseo_osmosis: {}", juno_addr_eliseo_osmosis);
        print!("\n");

        // test juno to terra and print the result
        let juno_addr_dan_terra =
            any_addr_to_prefix_addr(juno_addr_dan.to_string(), TERRA_BECH32_PREFIX)?;
        let juno_addr_eliseo_terra =
            any_addr_to_prefix_addr(juno_addr_eliseo.to_string(), TERRA_BECH32_PREFIX)?;
        println!("juno_addr_dan_terra:\t {}", juno_addr_dan_terra);
        println!("juno_addr_eliseo_terra: {}", juno_addr_eliseo_terra);
        print!("\n");

        // test juno to kujira and print the result
        let juno_addr_dan_kujira =
            any_addr_to_prefix_addr(juno_addr_dan.to_string(), KUJIRA_BECH32_PREFIX)?;
        let juno_addr_eliseo_kujira =
            any_addr_to_prefix_addr(juno_addr_eliseo.to_string(), KUJIRA_BECH32_PREFIX)?;
        println!("juno_addr_dan_kujira:\t {}", juno_addr_dan_kujira);
        println!("juno_addr_eliseo_kujira: {}", juno_addr_eliseo_kujira);
        print!("\n");

        Ok(())
    }
}
