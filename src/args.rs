use structopt::StructOpt;



// get command line args using structopt
#[derive(StructOpt, Debug)]
#[structopt(name="xpend")]
pub enum Args {
    Add {
        #[structopt(short)]
        amount: usize,
        #[structopt(short)]
        reason: String
    },

    Take {
        #[structopt(short)]
        amount: usize,
        #[structopt(short)]
        reason: String
    }
}


