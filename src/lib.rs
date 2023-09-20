


/// This function transfer the addr to a local neutron addr
pub fn any_addr_to_prefix_addr(addr: String, prefix: &str) -> Result<String, bech32::Error> {
    // TODO, test this snippet
    let (_hrp, data, _variant) = bech32::decode(&addr)?;
    let neutron_addr = bech32::encode(prefix, data, bech32::Variant::Bech32)?;
    Ok(neutron_addr)
}


#[cfg(test)]
mod tests {
    use super::*;

    const JUNO_BECH32_PREFIX: &str = "juno";
    const NEUTRON_BECH32_PREFIX: &str = "neutron";
    const OSMOSIS_BECH32_PREFIX: &str = "osmo";
    const TERRA_BECH32_PREFIX: &str = "terra";
    const KUJIRA_BECH32_PREFIX: &str = "kujira";

    #[test]
    fn test_any_addr_to_neutron() {

        let juno_addr_dan = "juno14wd9r77c2rlau7r8lta6mh8nqsvqq00rxha3fl";
        let juno_addr_eliseo = "juno1e7faq9nlpms0hfddd8vqqgval478shukpj8c50";

        // test juno to neutron and print the result
        let juno_addr_dan_neutron = any_addr_to_prefix_addr(juno_addr_dan.to_string(), NEUTRON_BECH32_PREFIX).unwrap();
        let juno_addr_eliseo_neutron = any_addr_to_prefix_addr(juno_addr_eliseo.to_string(), NEUTRON_BECH32_PREFIX).unwrap();
        println!("juno_addr_dan_neutron: \t{}", juno_addr_dan_neutron);
        println!("juno_addr_eliseo_neutron: {}", juno_addr_eliseo_neutron);
        print!("\n");

        // test juno to osmosis and print the result
        let juno_addr_dan_osmosis = any_addr_to_prefix_addr(juno_addr_dan.to_string(), OSMOSIS_BECH32_PREFIX).unwrap();
        let juno_addr_eliseo_osmosis = any_addr_to_prefix_addr(juno_addr_eliseo.to_string(), OSMOSIS_BECH32_PREFIX).unwrap();
        println!("juno_addr_dan_osmosis:\t {}", juno_addr_dan_osmosis);
        println!("juno_addr_eliseo_osmosis: {}", juno_addr_eliseo_osmosis);
        print!("\n");

        // test juno to terra and print the result
        let juno_addr_dan_terra = any_addr_to_prefix_addr(juno_addr_dan.to_string(), TERRA_BECH32_PREFIX).unwrap();
        let juno_addr_eliseo_terra = any_addr_to_prefix_addr(juno_addr_eliseo.to_string(), TERRA_BECH32_PREFIX).unwrap();
        println!("juno_addr_dan_terra:\t {}", juno_addr_dan_terra);
        println!("juno_addr_eliseo_terra: {}", juno_addr_eliseo_terra);
        print!("\n");

        // test juno to kujira and print the result
        let juno_addr_dan_kujira = any_addr_to_prefix_addr(juno_addr_dan.to_string(), KUJIRA_BECH32_PREFIX).unwrap();
        let juno_addr_eliseo_kujira = any_addr_to_prefix_addr(juno_addr_eliseo.to_string(), KUJIRA_BECH32_PREFIX).unwrap();
        println!("juno_addr_dan_kujira:\t {}", juno_addr_dan_kujira);
        println!("juno_addr_eliseo_kujira: {}", juno_addr_eliseo_kujira);
        print!("\n");


    }
    
}
