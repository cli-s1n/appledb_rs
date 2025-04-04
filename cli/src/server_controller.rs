use anyhow::{Result, bail};
use appledb_common::IPSWEntitlements;
use appledb_common::api_models::{EntitlementsInsertionStatus, ServerErrorResponse};
use appledb_common::config::ListenMode;
use appledb_common::db_models::OperatingSystem;
use appledb_common::routes::{ADMIN_ROUTES, POST_EXECUTABLE_ENTITLEMENTS_ROUTE, PublicRoutes};
use reqwest::{Client, ClientBuilder, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::net::SocketAddr;

macro_rules! response_to_result {
    ($response:expr) => {{
        match $response.status() {
            StatusCode::OK => Ok($response.json::<T>().await?),
            _ => {
                let error_response: ServerErrorResponse = $response.json().await?;
                bail!(format!("Server error: {}", error_response.reason))
            }
        }
    }};
}

pub struct ServerController {
    client: Client,
    address: SocketAddr,
}

impl ServerController {
    pub fn new(listen_mode: ListenMode) -> Result<Self> {
        let address = match listen_mode {
            ListenMode::SocketAddr(socket_addr) => socket_addr,
            ListenMode::UnixSocket(_) => bail!("unix socket not supported yet"),
        };

        let client = ClientBuilder::new().gzip(true).build()?;

        Ok(Self { client, address })
    }

    fn gen_url<S: AsRef<str>>(&self, path: S) -> String {
        format!("http://{}{}", self.address, path.as_ref())
    }

    fn gen_admin_url<S: AsRef<str>>(&self, path: S) -> String {
        self.gen_url(format!("{ADMIN_ROUTES}{}", path.as_ref()))
    }

    fn gen_public_url<S: AsRef<str>>(&self, path: S) -> String {
        self.gen_url(format!("{}{}", PublicRoutes::route_prefix(), path.as_ref()))
    }

    async fn get<T: DeserializeOwned>(&self, url: String) -> Result<T> {
        response_to_result!(self.client.get(&url).send().await?)
    }

    async fn post<D: Serialize, T: DeserializeOwned>(&self, url: String, data: D) -> Result<T> {
        response_to_result!(self.client.post(&url).json(&data).send().await?)
    }

    pub async fn get_operating_systems(&self) -> Result<Vec<OperatingSystem>> {
        self.get(self.gen_public_url(PublicRoutes::GetOperatingSystems.to_string()))
            .await
    }

    pub async fn post_executable_entitlements(
        &self,
        entitlements: IPSWEntitlements,
    ) -> Result<EntitlementsInsertionStatus> {
        return self
            .post(
                self.gen_admin_url(POST_EXECUTABLE_ENTITLEMENTS_ROUTE),
                entitlements,
            )
            .await;
    }
}
