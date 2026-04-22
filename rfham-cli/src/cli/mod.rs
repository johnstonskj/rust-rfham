use crate::{
    COMMAND_NAME, OnceCommand, OnceCommandWith,
    command::{completions::GenerateCompletions, external::RunExternalSubCommand},
    error::CliError,
};
use clap::{ArgAction, Parser, Subcommand};
use clap_complete::Shell;
use colorchoice::ColorChoice;
use colorchoice_clap::Color;
use std::process::ExitCode;
use tracing::{instrument, level_filters::LevelFilter, trace};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Parser)]
#[command(name = COMMAND_NAME)]
#[command(about = "Rusty Ham CLI for ham radio things", long_about = None)]
pub struct Cli {
    /// Increase logging verbosity by one level per occurance.
    #[arg(
        long,
        short = 'v',
        action = ArgAction::Count,
        global = true,
    )]
    verbose: u8,

    /// Decrease logging verbosity by one level per occurance.
    #[arg(
        long,
        short = 'q',
        action = ArgAction::Count,
        global = true,
        conflicts_with = "verbose",
    )]
    quiet: u8,

    #[command(flatten)]
    color: Color,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Show or calculate antenna details.
    #[command(subcommand)]
    Antenna(AntennaCommands),

    /// Output a formatted band plan for a given country.
    #[command(subcommand)]
    BandPlan(BandPlanCommands),

    /// Callsign commands.
    #[command(subcommand)]
    CallSign(CallSignCommands),

    /// Generate command completions for a number of shells.
    Completions {
        /// Shell to generate completions for (defaults to $SHELL).
        shell: Option<Shell>,
    },

    /// Show, or initialize, or edit, the current configuration.
    #[command(subcommand)]
    Config(ConfigCommands),

    #[command(external_subcommand)]
    External(#[arg(num_args = 1..)] Vec<String>),
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Globals {
    verbose_arg_count: u8,
    quiet_arg_count: u8,
    color_choice: ColorChoice,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl OnceCommand for Cli {
    type Output = ExitCode;
    type Error = CliError;

    #[instrument(name = "cli")]
    fn execute(self) -> Result<Self::Output, Self::Error> {
        trace!("Setting color globals to `{:?}`", self.color.as_choice());
        self.color.write_global();
        match self.color.as_choice() {
            ColorChoice::Always | ColorChoice::AlwaysAnsi => colored::control::set_override(true),
            ColorChoice::Never => colored::control::set_override(false),
            ColorChoice::Auto => {}
        }
        self.command.execute_with(Globals {
            verbose_arg_count: self.verbose,
            quiet_arg_count: self.quiet,
            color_choice: self.color.as_choice(),
        })
    }
}

impl Cli {
    pub fn max_level_filter(&self) -> LevelFilter {
        const DEFAULT_LEVEL_ERROR: u8 = 1;
        let offset = self.verbose as i16 - self.quiet as i16;
        match i16::from(DEFAULT_LEVEL_ERROR).saturating_add(offset) {
            i16::MIN..=0 => LevelFilter::OFF,
            1 => LevelFilter::ERROR,
            2 => LevelFilter::WARN,
            3 => LevelFilter::INFO,
            4 => LevelFilter::DEBUG,
            5..=i16::MAX => LevelFilter::TRACE,
        }
    }
}

impl OnceCommandWith for Commands {
    type Output = ExitCode;
    type Error = CliError;
    type Value = Globals;

    fn execute_with(self, globals: Globals) -> Result<Self::Output, Self::Error> {
        match self {
            Self::Antenna(cmds) => cmds.execute(),
            Self::BandPlan(cmds) => cmds.execute(),
            Self::CallSign(cmds) => cmds.execute(),
            Self::Config(cmds) => cmds.execute(),
            // One-shot commands.
            Self::Completions { shell } => GenerateCompletions::new(shell).execute(),
            Self::External(args) => RunExternalSubCommand::new(
                args[0].clone(),
                if args.len() > 1 { &args[1..] } else { &[] },
            )
            .with_verbosity_level(globals.verbose_arg_count)
            .with_quietness_level(globals.quiet_arg_count)
            .with_color_choice(globals.color_choice)
            .execute(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

mod antennas;
use antennas::AntennaCommands;
mod bands;
use bands::BandPlanCommands;
mod callsign;
use callsign::CallSignCommands;
mod config;
use config::ConfigCommands;
