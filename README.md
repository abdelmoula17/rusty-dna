# rusty-dna

rusty-dna is a small Rust library that normalizes raw genotype data exported by consumer DNA testing providers. Given a DNA file and a provider-specific parser, it emits a `HashMap<String, Snp>` keyed by rsid so the rest of your application can work with a consistent representation or serialize the results.

The project draws inspiration from the JavaScript tool [genomejs/dna2json](https://github.com/genomejs/dna2json), reimagined here for Rust-centric workflows.

## Features
- Unified `dna_parser` entry point that trims input, skips headers/comments, and collects SNPs into a map keyed by rsid.
- Built-in adapters for popular providers (`Ancestry`, `GenesForGood`, `TwentyThreeAndMe`) that recognise each vendor file format and clean up provider-specific quirks like chromosome aliases or no-call markers.
- Extensible `DnaParser` trait so you can plug in additional providers or custom processing.

## Installation

Once the crate is published to crates.io you can add it with:

```bash
cargo add rusty-dna
```

Or update your `Cargo.toml` manually:

```toml
[dependencies]
rusty-dna = "0.1.0"
```

## Usage

```rust
use rusty_dna::{
    dna_parser,
    providers::twenty_three_and_me::TwentyThreeAndMe,
};

fn main() -> Result<(), String> {
    let raw = std::fs::read_to_string("23andme_raw_data.txt")?;
    let snps = dna_parser(raw, TwentyThreeAndMe)?;

    let rs1 = snps.get("rs1").expect("expected rs1 entry");
    println!("{} → {} {}", rs1.rsid, rs1.chromosome, rs1.genotype);

    Ok(())
}
```

Each provider parser expects tab-separated values with the columns `rsid`, `chromosome`, `position`, and genotype information. Lines beginning with `#` or the header `rsid` are ignored automatically.

### Implementing a custom provider

Implement the `DnaParser` trait and supply it to `dna_parser` to handle additional file formats:

```rust
use rusty_dna::{contracts::DnaParser, dna_parser, structs::Snp};

struct MyProvider;

impl DnaParser for MyProvider {
    fn matches(&self, contents: &str) -> Result<(), String> {
        if contents.contains("My Provider Signature") {
            Ok(())
        } else {
            Err("unsupported file".into())
        }
    }

    fn parse(&self, line: &str) -> Result<Snp, String> {
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 4 {
            return Err("invalid line".into());
        }
        Ok(Snp {
            rsid: cols[0].into(),
            chromosome: cols[1].into(),
            position: cols[2].parse().map_err(|_| "bad position".to_string())?,
            genotype: cols[3].replace('-', "?"),
        })
    }
}

fn main() -> Result<(), String> {
    let raw = include_str!("my_provider.txt").to_string();
    let snps = dna_parser(raw, MyProvider)?;
    println!("Parsed {} SNPs", snps.len());
    Ok(())
}
```

## Testing

Run the unit test suite with:

```bash
cargo test
```

The tests cover the core parser flow and the bundled provider implementations.

## License

Licensed under the [MIT License](LICENSE).
