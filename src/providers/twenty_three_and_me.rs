use crate::{contracts::DnaParser, structs::Snp};

pub struct TwentyThreeAndMe;
impl DnaParser for TwentyThreeAndMe {
    fn matches(&self, lines: &str) -> Result<(), String> {
        if lines.contains("23andMe") {
            Ok(())
        } else {
            Err("Not a 23andMe file".to_string())
        }
    }

    fn parse(&self, line: &str) -> Result<Snp, String> {
        let split_line: Vec<&str> = line.split('\t').collect();
        if split_line.len() < 4 {
            return Err("Invalid line format".to_string());
        }
        let mut snp = Snp {
            rsid: split_line[0].to_string(),
            chromosome: split_line[1].to_string(),
            position: split_line[2].parse().unwrap_or(0),
            genotype: split_line[3].to_string(),
        };
        snp.genotype = snp.genotype.replace('-', "?"); // no-calls
        snp.genotype = snp.genotype.replace('D', "-"); // deletions
        Ok(snp)
    }
}
