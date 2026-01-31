use std::{path::Path, process::Command};

use log::{debug, info, trace};

use crate::{
    CONFIG,
    template::{ModEntry, ModEntryType, ModProvider, Template},
};

macro_rules! config_or_struct {
    ($template:expr, $field:ident) => {
        CONFIG.$field.as_ref().unwrap_or(&$template.$field)
    };
    ($template:expr, $field:ident?) => {
        CONFIG.$field.as_ref().or($template.$field.as_ref())
    };
}

macro_rules! append_arg {
    ($command:ident, $field:ident) => {
        $command.arg(concat!("--", stringify!($field))).arg($field);
    };
    ($command:ident, $field:ident, $alias:literal) => {
        $command.arg(concat!("--", $alias)).arg($field);
    };
    ($command:ident, $field:ident?) => {
        if let Some($field) = $field {
            $command.arg(concat!("--", stringify!($field))).arg($field);
        }
    };
    ($command:ident, $field:ident?, $alias:literal) => {
        if let Some($field) = $field {
            $command.arg(concat!("--", $alias)).arg($field);
        }
    };
    ($command:ident, if $field:ident) => {
        if $field {
            $command.arg(concat!("--", stringify!($field)));
        }
    };
    ($command:ident, if $field:ident, $alias:literal) => {
        if #field {
            $command.arg(concat!("--", $alias));
        }
    };
}

pub struct PackwizWrapper(Box<Path>);
impl PackwizWrapper {
    pub fn new(path: Box<Path>) -> Self {
        if !path.exists() {
            std::fs::create_dir(&path).expect("Couldn't create packwiz pack directory");
        }
        PackwizWrapper(path)
    }

    pub fn init_project(&self, template: &Template) {
        let name = config_or_struct!(&template.info, name);
        let mc_version = config_or_struct!(template.info, mc_version);
        let author = config_or_struct!(template.info, author?);
        let version = config_or_struct!(template.info, version?);
        let yes = CONFIG.yes;

        let mut command = Command::new("packwiz");

        command.arg("init");
        command.current_dir(std::fs::canonicalize(self.0.as_ref()).unwrap());

        append_arg!(command, name);
        append_arg!(command, mc_version, "mc-version");
        append_arg!(command, author?);
        append_arg!(command, version?);
        append_arg!(command, if yes);

        info!("Initializing packwiz project");
        debug!("Executing command:\n {:?}", command);

        let _ = command
            .spawn()
            .expect("Couldn't run packwiz command")
            .wait();
    }

    pub fn add_mod(&self, template: &Template, entry: &ModEntry) {
        let provider = if let ModEntry::Advance { provider, .. } = entry {
            provider.unwrap_or(template.info.provider)
        } else {
            template.info.provider
        };

        match entry {
            ModEntry::Simple(id)
            | ModEntry::Advance {
                ty: ModEntryType::Single,
                id,
                ..
            } => {
                Self::spawn_add_process(id, provider, self.0.as_ref());
            }
            ModEntry::Advance {
                ty: ModEntryType::List { mods } | ModEntryType::Pick { mods },
                ..
            } => {
                for id in mods {
                    Self::spawn_add_process(id, provider, self.0.as_ref());
                }
            }
        }
    }

    fn spawn_add_process(id: &str, provider: ModProvider, pwd: &Path) {
        let _ = Command::new("packwiz")
            .current_dir(std::fs::canonicalize(pwd).unwrap())
            .arg(provider.as_ref())
            .arg("add")
            .arg(id)
            .spawn()
            .expect("Couldn't run packwiz command")
            .wait();
    }
}
