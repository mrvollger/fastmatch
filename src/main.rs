use fastmatch;
use std::io;
use std::io::*;
use std::path::PathBuf;
use structopt::StructOpt;
use bio::io::fasta;
use bio::pattern_matching::myers::Myers;




#[derive(Debug, StructOpt)]
#[structopt(name = "fastamatch", 
    about = "A wrapper around Myer's bit parallel algorithm to find fast matches. The positons reported are the end of the match.")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    #[structopt(short = "e", long = "edits", default_value = "0",
	help = "Number of edits allowed between pattern and text for a match.")]
    edits: u8,

    #[structopt(short = "f", long = "fasta",
	help = "Search for the pattern in this fasta file.",
	parse(from_os_str)  )]
    fasta: PathBuf,

    #[structopt(short = "p", long = "pattern",
	help = "Pattern to search for. Must be less than 64 characters.")]
    pattern: String,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() {
    let opts = Opt::from_args();
    let reader = fasta::Reader::from_file(opts.fasta).expect("Bad fasta!");
    let pattern = opts.pattern.as_bytes();
    let pattern_fc  = Myers::<u64>::new(pattern);
    let pattern_rc = Myers::<u64>::new(bio::alphabets::dna::revcomp(pattern));

    for record in reader.records(){
        let record = record.unwrap();
        let seq = record.seq();
        let name = record.id();
        let occ_fc = pattern_fc.find_all_end(seq, opts.edits);
        let occ_rc = pattern_rc.find_all_end(seq, opts.edits);
        let occ : Vec<(usize, u8)> = occ_fc.chain(occ_rc).collect();
        fastmatch::write_ends(name, occ).expect("Write error!");
    }
}

