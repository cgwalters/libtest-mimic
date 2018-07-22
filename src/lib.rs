#[macro_use]
extern crate structopt;

use std::{
    fmt,
    str::FromStr,
};

/// Command line arguments.
///
/// This type represents everything the user can specify via CLI args. The main
/// method is `from_args()` which reads the global `std::env::args()` and
/// parses them into this type. The CLI is very similar to the one from the
/// native test harness. There are minor differences, mainly in the help
/// output.
///
/// **Note**: just because all CLI args can be parsed, doesn't mean that they
/// are used. Check [`run_tests`] for information on which arguments are used.
#[derive(StructOpt, Debug)]
#[structopt(
    template = "USAGE: [FLAGS] [OPTIONS] [FILTER]\n\n{all-args}\n\n\n{after-help}",
    raw(setting = "structopt::clap::AppSettings::DisableVersion"),
    after_help = "By default, all tests are run in parallel. This can be altered with the \n\
        --test-threads flag or the RUST_TEST_THREADS environment variable when running \n\
        tests (set it to 1).\n\
        \n\
        All tests have their standard output and standard error captured by default. \n\
        This can be overridden with the --nocapture flag or setting RUST_TEST_NOCAPTURE \n\
        environment variable to a value other than \"0\". Logging is not captured by default.",
)]
pub struct Arguments {
    // ============== FLAGS ===================================================
    /// Determines if ignored tests should be run.
    #[structopt(long = "--ignored", help = "Run ignored tests")]
    pub ignored: bool,

    /// Run tests, but not benchmarks.
    #[structopt(long = "--test", help = "Run tests and not benchmarks")]
    pub test: bool,

    /// Run benchmarks, but not tests.
    #[structopt(long = "--bench", help = "Run benchmarks instead of tests")]
    pub bench: bool,

    /// Only list all tests and benchmarks.
    #[structopt(long = "--list", help = "List all tests and benchmarks")]
    pub list: bool,

    /// If set, stdout/stderr are not captured during the test but are instead
    /// printed directly.
    #[structopt(
        long = "--nocapture",
        help = "don't capture stdout/stderr of each task, allow printing directly",
    )]
    pub nocapture: bool,

    /// If set, filters are matched exactly rather than by substring.
    #[structopt(
        long = "--exact",
        help = "Exactly match filters rather than by substring",
    )]
    pub exact: bool,

    // TODO: can only be set if another isnt
    /// If set, display only one character per test instead of one line.
    ///
    /// This is an alias for `--format=terse`. If this is set, `format` is
    /// `None`.
    #[structopt(
        short = "q",
        long = "--quiet",
        conflicts_with = "format",
        help = "Display one character per test instead of one line. Alias to --format=terse",
    )]
    pub quiet: bool,


    // ============== OPTIONS =================================================
    /// Number of threads used for parallel testing.
    #[structopt(
        long = "--test-threads",
        help = "Number of threads used for running tests in parallel"
    )]
    pub num_threads: Option<u32>,

    /// Path of the logfile. If specified, everything will be written into the
    /// file instead of stdout.
    #[structopt(
        long = "--logfile",
        value_name = "PATH",
        help = "Write logs to the specified file instead of stdout",
    )]
    pub logfile: Option<String>,

    /// A list of filters. Tests whose names contain parts of any of these
    /// filters are skipped.
    #[structopt(
        long = "--skip",
        value_name = "FILTER",
        raw(number_of_values = "1"),
        help = "Skip tests whose names contain FILTER (this flag can be used multiple times)",
    )]
    pub skip: Vec<String>,

    /// Specifies whether or not to color the output.
    #[structopt(
        long = "--color",
        raw(possible_values = r#"&["auto", "always", "never"]"#),
        value_name = "auto|always|never",
        help = "Configure coloring of output: \n\
            - auto = colorize if stdout is a tty and tests are run on serially (default)\n\
            - always = always colorize output\n\
            - never = never colorize output\n",
    )]
    pub color: Option<ColorSetting>,

    /// Specifies the format of the output.
    #[structopt(
        long = "--format",
        raw(possible_values = r#"&["pretty", "terse", "json"]"#),
        value_name = "pretty|terse|json",
        help = "Configure formatting of output: \n\
            - pretty = Print verbose output\n\
            - terse = Display one character per test\n\
            - json = Output a json document\n",
    )]
    pub format: Option<FormatSetting>,


    // ============== POSITIONAL VALUES =======================================
    /// Filter string. Only tests which contain this string are run.
    #[structopt(
        name = "FILTER",
        help = "The FILTER string is tested against the name of all tests, and only those tests \
            whose names contain the filter are run.",
    )]
    pub filter_string: Option<String>,
}

impl Arguments {
    /// Parses the CLI arguments given to the application into `Arguments`.
    ///
    /// If the parsing fails (due to incorrect CLI args), an error is shown and
    /// the application exits. If help is requested (`-h` or `--help`), a help
    /// message is shown and the application exits, too.
    pub fn from_args() -> Self {
        structopt::StructOpt::from_args()
    }
}

/// Possible values for the `--color` option.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSetting {
    /// Colorize output if stdout is a tty and tests are run on serially
    /// (default).
    Auto,

    /// Always colorize output.
    Always,

    /// Never colorize output.
    Never,
}

impl FromStr for ColorSetting {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(ColorSetting::Auto),
            "always" => Ok(ColorSetting::Always),
            "never" => Ok(ColorSetting::Never),
            _ => Err("foo"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatSetting {
    /// One line per test. Output for humans. (default)
    Pretty,

    /// One character per test. Usefull for test suites with many tests.
    Terse,

    /// Output as JSON.
    Json,
}

impl FromStr for FormatSetting {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pretty" => Ok(FormatSetting::Pretty),
            "terse" => Ok(FormatSetting::Terse),
            "json" => Ok(FormatSetting::Json),
            _ => Err("foo"),
        }
    }
}