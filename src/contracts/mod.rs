use super::structs::Snp;

pub trait DnaParser {
    fn matches(&self, lines: &str) -> Result<(), String>;
    fn parse(&self, line: &str) -> Result<Snp, String>;
}
