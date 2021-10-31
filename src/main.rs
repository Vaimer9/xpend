mod init;
mod args;

use args::Args;
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();
}
