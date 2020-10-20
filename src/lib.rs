use std::io;
use std::io::*;
use std::path::PathBuf;
use structopt::StructOpt;
use bio::io::fasta;
use bio::pattern_matching::myers::Myers;

/// This writes the results of pattern matching to stdout
pub fn write_ends(name : &str, occ : Vec<(usize, u8)> ) -> io::Result<()>{
    /// this is a doc string 
    ///
    let mut out = io::stdout();
    write!(out, "{}:", name)?;
    for (end, _edits) in occ{
        write!(out, "{},", end)?;
    }
    write!(out, "\n")?;
    Ok(())
}
