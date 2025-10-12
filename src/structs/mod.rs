#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Snp {
    pub rsid: String,
    pub chromosome: String,
    pub position: u32,
    pub genotype: String,
}
