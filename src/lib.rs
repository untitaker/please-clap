// DOCS

/// Process a `clap::ArgMatches` with pattern-matching-like syntax.
///
/// ```
/// # #[macro_use] extern crate please_clap;
/// # extern crate clap;
/// use self::clap::*;
/// # fn main() {
///    let matches = App::new("test")
///        .subcommand(SubCommand::with_name("sub")
///            .subcommand(SubCommand::with_name("subsub")
///                .arg(Arg::with_name("TEST_ARG").index(1))))
///        .subcommand(SubCommand::with_name("othersub"))
///        .get_matches_from(vec!["test", "sub", "subsub", "fooarg"]);
///
///    let mut called = false;
///    clap_dispatch!(matches; {
///        sub(sub_matches) => clap_dispatch!(sub_matches; {
///            subsub(_, TEST_ARG as test_arg) => {
///                assert_eq!(test_arg, "fooarg");
///                called = true;
///            }
///        }),
///        othersub() => { panic!("Should not have been called."); }
///    });
///
///    assert!(called);
/// # }
/// ```
#[macro_export]
macro_rules! clap_dispatch {
    ($matches:expr; { $( $name:ident $match_arm_args:tt => $callback:expr ),* }) => {
        match $matches.subcommand_name() {
            $(
                Some(stringify!($name)) => {
                    let matches = $matches.subcommand_matches(stringify!($name)).unwrap();
                    clap_dispatch!(MATCH_ARM_ARGS, matches, $match_arm_args);
                    $callback;
                }
            )*
            Some(x) => {
                panic!("Internal error: Command not covered: {}", x);
            },
            None => {
                println!("Subcommand required. See --help for help.");
                ::std::process::exit(1);
            }
        }
    };

    (MATCH_ARM_ARGS, $matches:ident, ( $matches_name:pat, $($arg_name:ident as $varname:ident),* )) => {
        $(
            let $varname = $matches.value_of(stringify!($arg_name)).unwrap();
        )*
        let $matches_name = $matches;
    };

    // Transform `foo(_) => ()` into `foo(_,) => ()`
    (MATCH_ARM_ARGS, $matches:ident, ( $matches_name:pat )) => { clap_dispatch!(MATCH_ARM_ARGS, $matches, ($matches_name,)) };

    // Transform `foo() => ()` into `foo(_,) => ()`
    (MATCH_ARM_ARGS, $matches:ident, ()) => { clap_dispatch!(MATCH_ARM_ARGS, $matches, (_,)) };

}
