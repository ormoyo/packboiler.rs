use clap::ValueEnum;
use enumfmt::EnumFmt;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{collections::HashMap, fmt::Display, path::Path};
use strum::{AsRefStr, EnumIter};

type ModId = String;
pub const TEMPLATE_EXTENSION: &str = "packtemplate";

#[derive(AsRefStr, Serialize, Deserialize, ValueEnum, Clone, Copy, Debug)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ModProvider {
    Curseforge,
    Modrinth,
}

#[derive(
    EnumIter, EnumFmt, Serialize, Deserialize, ValueEnum, Clone, Copy, PartialEq, Eq, Debug,
)]
#[serde(rename_all = "lowercase")]
#[enumfmt(
    as_str = "{lowercase}",
    as_long_arg = "{lowercase}-version",
    as_value_name = "{SCREAMING_SNAKE_CASE}_VERSION"
)]
pub enum ModLoader {
    Forge,
    Fabric,
    Neoforge,
    Quilt,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum EnableModules {
    All,
    #[serde(untagged)]
    List(Vec<String>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ModEntryType {
    Single,
    List { mods: Vec<ModId> },
    Pick { mods: Vec<ModId> },
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged, rename_all = "lowercase")]
pub enum ModEntry {
    Simple(ModId),
    Advance {
        #[serde(flatten)]
        ty: ModEntryType,
        id: String,
        provider: Option<ModProvider>,
        desc: Option<String>,
    },
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Template {
    #[serde(flatten)]
    pub info: TemplateInfo,
    pub imports: Option<HashMap<String, String>>,
    pub enable_modules: Option<EnableModules>,
    pub modules: HashMap<String, Module>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TemplateInfo {
    pub name: String,
    pub desc: Option<String>,
    pub author: Option<String>,
    pub version: Option<String>,
    pub provider: ModProvider,
    #[serde(rename = "loader")]
    pub modloader: ModLoader,
    pub mc_version: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Module {
    #[serde(flatten)]
    pub import: Option<ModuleImport>,

    #[serde(rename = "name")]
    pub display_name: String,
    pub desc: Option<String>,
    pub mods: Vec<ModEntry>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModuleImport {
    #[serde(rename = "from")]
    pub import: String,
    pub picks: Option<HashMap<String, Vec<ModId>>>,
    pub append_mods: Option<bool>,
}

impl Template {
    pub fn load_from_file(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();

        serde_yaml::from_str(
            &std::fs::read_to_string(path)
                .unwrap_or_else(|err| panic!("Couldn't read file: {:?}\n {err}", path.as_os_str())),
        )
        .unwrap_or_else(|err| {
            panic!(
                "Couldn't create template from file: {:?} {err}",
                path.as_os_str()
            )
        })
    }

    fn merge_modules_with_imports(&mut self) {}
}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.display_name)?;
        Ok(())
    }
}
