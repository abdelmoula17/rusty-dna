use std::collections::HashMap;

use crate::{contracts::DnaParser, structs::Snp};

pub struct Ancestry;
impl DnaParser for Ancestry {
    fn matches(&self, lines: &str) -> Result<(), String> {
        print!("Checking Ancestry match...\n");
        if lines.contains("AncestryDNA") {
            Ok(())
        } else {
            Err("Not an Ancestry file".to_string())
        }
    }

    fn parse(&self, line: &str) -> Result<Snp, String> {
        let chromosome_replacement =
            HashMap::from([("23", "X"), ("24", "Y"), ("25", "MT"), ("26", "XY")]);
        let split_line: Vec<&str> = line.split('\t').collect();
        if split_line.len() < 4 {
            return Err("Invalid line format".to_string());
        }
        let mut snp = Snp {
            rsid: split_line[0].to_string(),
            chromosome: split_line[1].to_string(),
            position: split_line[2].parse().unwrap_or(0),
            genotype: format!("{} {}", split_line[3], split_line[4]),
        };
        if let Some(&replacement) = chromosome_replacement.get(snp.chromosome.as_str()) {
            snp.chromosome = replacement.to_string();
        }

        snp.genotype = snp.genotype.replace('0', "?"); // no-calls
        snp.genotype = snp.genotype.replace('D', "-"); // deletions
        Ok(snp)
    }
}
