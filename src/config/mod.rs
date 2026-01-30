mod modloader;

use clap::Parser;
use modloader::ModloaderConfig;

#[derive(Parser)]
#[command(version, about)]
pub struct PackConfig {
    pub path: String,
    #[arg(short, long)]
    pub template: Option<String>,
    #[arg(id = "pack_name", value_name = "PACK_NAME", long)]
    pub name: Option<String>,
    #[arg(id = "pack_author", value_name = "PACK_AUTHOR", long)]
    pub author: Option<String>,
    #[arg(long, visible_alias = "mc")]
    pub mc_version: Option<String>,
    #[arg(id = "pack_version", value_name = "PACK_VERSION", long)]
    pub version: Option<String>,
    #[command(flatten)]
    pub modloader: ModloaderConfig,
    #[arg(short, long)]
    pub yes: bool,
}
