use anyhow::{anyhow, Error};
use cargo_metadata::MetadataCommand;
use fehler::throws;
use regex::Regex;
use std::path::PathBuf;

pub struct WorkspaceMember {
    pub name: String,
    pub version: String,
    pub path: PathBuf,
}

#[throws]
pub fn web_worker_workspace_members() -> Vec<WorkspaceMember> {
    let package_repr_regex =
        Regex::new(r"^(?P<name>\S+)\s(?P<version>\S+)\s\(path\+file://(?P<path>\S+)\)$")?;

    MetadataCommand::new()
        .no_deps()
        .exec()?
        .workspace_members
        .into_iter()
        .filter_map(|package_id| {
            let Some(captures) = package_repr_regex.captures(&package_id.repr) else {
                return Some(Err(anyhow!(
                    "Failed to parse workspace member with {package_id:?}"
                )));
            };
            let name = &captures["name"];
            name.ends_with("web_worker")
                .then(|| WorkspaceMember {
                    name: name.to_owned(),
                    version: captures["version"].to_owned(),
                    path: PathBuf::from(&captures["path"]),
                })
                .map(Ok)
        })
        .collect::<Result<_, _>>()?
}
