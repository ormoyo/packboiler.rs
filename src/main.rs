use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use clap::Parser;
use const_format::concatcp;
use dialoguer::{Input, MultiSelect};

mod config;
mod packwiz_wrapper;
mod path_completion;
mod template;

use config::PackConfig;
use packwiz_wrapper::PackwizWrapper;
use path_completion::PathCompletion;
use template::*;

pub static CONFIG: LazyLock<PackConfig> = LazyLock::new(PackConfig::parse);
fn main() {
    let _ = &*CONFIG;
    let file_validation = |input: &String| {
        let path = std::path::Path::new(input);
        match path.try_exists() {
            Ok(true) if path.is_file() => {
                if path
                    .extension()
                    .is_some_and(|ext| ext == TEMPLATE_EXTENSION)
                {
                    Ok(())
                } else {
                    Err(concatcp!("File must have extension: ", TEMPLATE_EXTENSION))
                }
            }
            Ok(true) => Err("Path is not a file"),
            Ok(false) => Err("File does not exist"),
            Err(_) => Err("File cannot be accessed or doesn't exist"),
        }
    };

    env_logger::init();

    let file_completion = PathCompletion;
    let template_path: Cow<'static, str> = CONFIG
        .template
        .as_deref()
        .map(Cow::Borrowed)
        .unwrap_or_else(|| {
            Cow::Owned(
                Input::new()
                    .with_prompt("Enter template path")
                    .validate_with(file_validation)
                    .completion_with(&file_completion)
                    .interact_text()
                    .unwrap(),
            )
        });
    let template_path: &Path = Path::new(template_path.as_ref());
    let template = Template::load_from_file(template_path);

    let project_path: PathBuf = CONFIG.path.clone().into();
    let init_project = if project_path.exists() {
        let path = project_path.join("pack.toml");
        !path.exists()
    } else {
        true
    };

    let packwiz_wrapper = PackwizWrapper::new(project_path.into_boxed_path());
    if init_project {
        packwiz_wrapper.init_project(&template);
    }

    let enabled_modules: Vec<&str> = if let Some(EnableModules::All) = template.enable_modules {
        template.modules.keys().map(String::as_str).collect()
    } else {
        let mut vec = if let Some(EnableModules::List(ref modules)) = template.enable_modules {
            modules.iter().map(String::as_str).collect()
        } else {
            Vec::new()
        };

        let a: Vec<&str> = template
            .modules
            .iter()
            .filter(|(id, _)| !vec.contains(&id.as_str()))
            .map(|(_, module)| module.display_name.as_str())
            .collect();
        let enabled_modules = MultiSelect::new()
            .with_prompt("Toggle modules:")
            .items(a.as_slice())
            .interact()
            .unwrap();

        enabled_modules
            .into_iter()
            .map(|i| a[i])
            .for_each(|str| vec.push(str));
        vec
    };

    for module in enabled_modules
        .iter()
        .filter_map(|m| template.modules.get(*m))
    {
        for mod_entry in &module.mods {
            packwiz_wrapper.add_mod(&template, mod_entry);
        }
    }
}
