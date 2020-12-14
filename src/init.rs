use clap::{clap_app, ArgMatches};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref OPTS: ArgMatches<'static> = {
        clap_app!(bcrypt =>
                  (version: "2.0")
                  (bin_name: "bcrypt")
                  (author: "Cheng JIANG <alex_cj96@foxmail.com>")
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
        )
        .get_matches()
    };
}
