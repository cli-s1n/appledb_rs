use std::{path::PathBuf, str::FromStr};

use crate::parsers::{FrameworksParser, IPSWParser};
use crate::utils::parse_macho;
use crate::{
    ipsw_executables::IPSWExecutablesIterator, models::FullSubcommand, parsers::EntitlementsParser,
    server_controller::ServerController,
};
use anyhow::Result;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn parse_full_subcommand(
    server_url: String,
    full_subcommand: FullSubcommand,
) -> Result<()> {
    match full_subcommand {
        FullSubcommand::Parse {
            mount_point,
            platform,
            version,
            model_code,
        } => {
            log::info!(
                "IPSW has platform={platform}, model_code={model_code} and version={version}"
            );

            let mut entitlements_parser =
                EntitlementsParser::new(platform.clone().into(), &model_code, &version);

            let mut frameworks_parser =
                FrameworksParser::new(platform.into(), &model_code, &version);

            for entry in IPSWExecutablesIterator::new(&mount_point).flatten() {
                let stripped_path = entry.strip_prefix(&mount_point)?;
                let full_absolute_path = match &stripped_path.is_absolute() {
                    true => stripped_path.to_path_buf(),
                    false => PathBuf::from_str("/")?.join(stripped_path),
                };

                let mut macho_file = File::open(&entry).await?;
                let mut macho_bin_data = Vec::new();
                macho_file.read_to_end(&mut macho_bin_data).await?;

                match parse_macho(&macho_bin_data) {
                    Ok(Some(macho)) => {
                        if let Err(e) = entitlements_parser
                            .parse_file(&full_absolute_path, &macho)
                            .await
                        {
                            log::error!(
                                "got error while parsing file {}: {e}",
                                full_absolute_path.display()
                            );
                        }

                        if let Err(e) = frameworks_parser
                            .parse_file(&full_absolute_path, &macho)
                            .await
                        {
                            log::error!(
                                "got error while parsing file {}: {e}",
                                full_absolute_path.display()
                            );
                        }
                    }
                    Ok(None) => {
                        continue;
                    }
                    Err(e) => log::error!("error while parsing macho {}: {e}", entry.display()),
                }
            }

            let server_controller = ServerController::new(server_url)?;

            let entitlements_task_uuid =
                entitlements_parser.post_results(&server_controller).await?;

            log::info!("Received entitlements task UUID: {entitlements_task_uuid}",);

            let frameworks_task_uuid = frameworks_parser.post_results(&server_controller).await?;

            log::info!("Received frameworks task UUID: {frameworks_task_uuid}");

            Ok(())
        }
    }
}
