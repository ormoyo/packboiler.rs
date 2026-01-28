use crate::ModLoader;
use clap::{Arg, Args, FromArgMatches, value_parser};
use strum::IntoEnumIterator;

pub struct ModloaderConfig {
    pub ty: Option<ModLoader>,
    pub version: Option<String>,
}

impl FromArgMatches for ModloaderConfig {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let loader: Option<ModLoader> = matches.get_one("modloader").copied();
        if loader.is_none() {
            return Ok(ModloaderConfig {
                ty: None,
                version: None,
            });
        }

        let version_id: &'static str = loader.unwrap().as_str();
        let version: Option<String> = matches.get_one(version_id).cloned();
        Ok(ModloaderConfig {
            ty: loader,
            version,
        })
    }
    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        let loader: Option<ModLoader> = matches.get_one("modloader").copied();
        if loader.is_none() {
            return Ok(());
        }

        if self.ty.is_some_and(|ty| ty != loader.unwrap()) {
            return Err(clap::Error::new(clap::error::ErrorKind::InvalidValue));
        }

        let version: &'static str = loader.unwrap().as_str();
        let version: Option<String> = matches.get_one(version).cloned();

        self.ty = loader;
        self.version = version;

        Ok(())
    }
}

impl Args for ModloaderConfig {
    fn augment_args(mut cmd: clap::Command) -> clap::Command {
        cmd = cmd.arg(
            Arg::new("modloader")
                .value_name("MODLOADER")
                .value_parser(value_parser!(ModLoader))
                .long("modloader"),
        );

        for ty in ModLoader::iter() {
            let name: &'static str = ty.as_value_name();
            let version: &'static str = ty.as_long_arg();

            let id: &'static str = ty.as_str();
            let arg = Arg::new(id).required(false).value_name(name).long(version);

            cmd = cmd.arg(arg);
        }
        cmd
    }
    fn augment_args_for_update(mut cmd: clap::Command) -> clap::Command {
        cmd = cmd.arg(
            Arg::new("modloader")
                .value_name("MODLOADER")
                .value_parser(value_parser!(ModLoader))
                .long("modloader"),
        );

        for ty in ModLoader::iter() {
            let name: &'static str = ty.as_value_name();
            let version: &'static str = ty.as_long_arg();

            let id: &'static str = ty.as_str();
            let arg = Arg::new(id).required(false).value_name(name).long(version);

            cmd = cmd.arg(arg);
        }
        cmd
    }
}
