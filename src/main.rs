pub mod indexes;

use std::sync::Arc;

use clap::{CommandFactory, FromArgMatches, Parser};
use cloud_terrastodon_user_input::{FzfArgs, pick, pick_many};
use tracing::{debug, info};

use crate::indexes::get_indexes;

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Cli {
    #[arg(long, global = true, default_value = "false")]
    pub debug: bool,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::command();
    let cli = Cli::from_arg_matches(&cli.get_matches())?;

    tracing_subscriber::fmt::SubscriberBuilder::default()
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .with_target(false)
        .with_max_level(match cli.debug {
            true => tracing::level_filters::LevelFilter::DEBUG,
            false => tracing::level_filters::LevelFilter::INFO,
        })
        .with_writer(std::io::stderr)
        .init();

    info!("Hello, world!");
    debug!("Debug mode is {}", cli.debug);

    let index = pick(FzfArgs {
        choices: get_indexes(),
        header: Some("Select an index:".to_string()),
        ..Default::default()
    })?;
    info!("Selected index: {}", index);

    let packages = indexes::get_index_packages(index)?;
    let chosen_package = pick(FzfArgs {
        choices: packages,
        header: Some("Select package:".to_string()),
        ..Default::default()
    })?;
    info!("Selected package: {:?}", chosen_package);

    let versions = indexes::get_package_versions(Arc::new(chosen_package))?;
    let chosen_version = pick(FzfArgs {
        choices: versions,
        header: Some("Select version:".to_string()),
        ..Default::default()
    })?;
    info!("Selected version: {:?}", chosen_version);

    Ok(())
}
