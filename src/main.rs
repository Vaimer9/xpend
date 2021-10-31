mod init;
mod args;

use args::Args;
use structopt::StructOpt;

struct User {
    init: bool
    money: usize
    logs: String
}

impl User {
    fn new() -> Self {
        User {
            init: false,
            money: 0,
            logs: String::new()
        }
    }

    fn create_logs(mut self) -> Result<(), io::Error> {
        init::make_folder()?;
        init::make_file()?;
        self.init = true;
    }

    fn log_register() {}

}

fn main() {
    let args = Args::from_args();
}
