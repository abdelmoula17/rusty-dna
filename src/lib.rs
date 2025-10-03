use std::collections::HashMap;

use contracts::DnaParser;
use structs::Snp;
pub mod contracts;
pub mod providers;
pub mod structs;
pub fn dna_parser<T: DnaParser>(dna: String, provider: T) -> Result<HashMap<String, Snp>, String> {
    let lines: Vec<&str> = dna.trim().lines().collect();
    let provider_parser = provider;
    let mut snp_map: HashMap<String, Snp> = HashMap::new();
    if provider_parser.matches(dna.as_str()).is_ok() {
        for line in &lines {
            if line.starts_with("#") || line.starts_with("rsid") {
                continue;
            }
            let snp = provider_parser.parse(&line).unwrap();
            snp_map.insert(snp.rsid.clone(), snp);
        }
    } else {
        return Err("No matching DNA provider found".to_string());
    }
    Ok(snp_map)
}
