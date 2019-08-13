use clap::{clap_app, value_t_or_exit, ArgMatches};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref OPTS: ArgMatches<'static> = {
        clap_app!(bcrypt =>
                  (version: "1.0")
                  (bin_name: "bcrypt")
                  (author: "Cheng JIANG <cheng.jiang@ubudu.com>")
                  (about: "bcrypt hash generation/verification")
                  (@setting ColoredHelp)
                  (@setting ArgRequiredElseHelp)
                  (@setting SubcommandRequiredElseHelp)
                  (@subcommand generate =>
                        (@setting ColoredHelp)
                        (@setting ArgRequiredElseHelp)
                        (about: "bcrypt hash generation")
                        (@arg input: * -i --input  +takes_value "bcrypt hash input")
                        (@arg cost: default_value("12") -c --cost +takes_value "bcrypt hash cost")
                  )
                  (@subcommand verify =>
                        (@setting ColoredHelp)
                        (@setting ArgRequiredElseHelp)
                        (about: "bcrypt hash verification")
                        (@arg input: * -i --input +takes_value "bcrypt hash input")
                        (@arg hashed: * -h --hashed +takes_value "bcrypt hashed output")
                  )
        ).get_matches()
    };
    // pub static ref COST: u8 = { value_t_or_exit!(OPTS.value_of("cost"), u8) };
}
