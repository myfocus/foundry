use std::path::PathBuf;

use crate::cmd::{forge::build::BuildArgs, Cmd};
use clap::{Parser, ValueHint};
use foundry_config::Config;

use super::build::ProjectPathsArgs;

#[derive(Debug, Clone, Parser)]
pub struct FlattenArgs {
    #[clap(help = "The path to the contract to flatten.", value_hint = ValueHint::FilePath)]
    pub target_path: PathBuf,

    #[clap(
        long,
        short,
        help = "The path to output the flattened contract.",
        long_help = "The path to output the flattened contract. If not specified, the flattened contract will be output to stdout.",
        value_hint = ValueHint::FilePath
    )]
    pub output: Option<PathBuf>,

    #[clap(flatten, next_help_heading = "PROJECT OPTIONS")]
    project_paths: ProjectPathsArgs,
}

impl Cmd for FlattenArgs {
    type Output = ();
    fn run(self) -> eyre::Result<Self::Output> {
        let FlattenArgs { target_path, output, project_paths } = self;

        // flatten is a subset of `BuildArgs` so we can reuse that to get the config
        let build_args = BuildArgs {
            project_paths,
            out_path: Default::default(),
            compiler: Default::default(),
            names: false,
            sizes: false,
            ignored_error_codes: vec![],
            no_auto_detect: false,
            use_solc: None,
            offline: false,
            force: false,
            libraries: vec![],
            watch: Default::default(),
            via_ir: false,
            config_path: None,
        };

        let config = Config::from(&build_args);

        let paths = config.project_paths();
        let target_path = dunce::canonicalize(target_path)?;
        let flattened = paths
            .flatten(&target_path)
            .map_err(|err| eyre::Error::msg(format!("Failed to flatten the file: {}", err)))?;

        match output {
            Some(output) => {
                std::fs::create_dir_all(&output.parent().unwrap())?;
                std::fs::write(&output, flattened)?;
                println!("Flattened file written at {}", output.display());
            }
            None => println!("{}", flattened),
        };

        Ok(())
    }
}
