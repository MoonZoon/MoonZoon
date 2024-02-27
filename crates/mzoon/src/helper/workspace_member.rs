use anyhow::Error;
use cargo_metadata::MetadataCommand;
use fehler::throws;
use std::path::PathBuf;

pub struct WorkspaceMember {
    pub name: String,
    pub path: PathBuf,
}

#[throws]
pub fn web_worker_workspace_members() -> Vec<WorkspaceMember> {
    MetadataCommand::new()
        .no_deps()
        .exec()?
        .packages
        .into_iter()
        .filter(|package| package.name.ends_with("web_worker"))
        .map(|package| WorkspaceMember {
            name: package.name,
            path: {
                let mut path = package.manifest_path;
                path.pop();
                path.into()
            },
        })
        .collect()
}
