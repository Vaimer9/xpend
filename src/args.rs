use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structop(name="xpend")]
pub struct Args {
    Add {
        #[structop(short)]
        amount: usize,
        #[structop(short)]
        reason: String
    }

    Take {
        #[structop(short)]
        amount: usize,
        #[structop(short)]
        reason: String
    }
}


