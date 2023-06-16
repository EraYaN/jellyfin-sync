use leptos::ServerFnError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JellyfinServerMetadata {
    pub server_name: String,
    pub admin_user_id: uuid::Uuid,
}

#[derive(Debug, Default)]
pub struct JellyfinApi {
    conf: openapi::apis::configuration::Configuration,
    pub metadata: Option<JellyfinServerMetadata>,
}

impl JellyfinApi {
    pub fn init(base_path: &str, user_agent: &str) -> Self {
        JellyfinApi {
            conf: openapi::apis::configuration::Configuration {
                base_path: String::from(base_path),
                user_agent: Some(String::from(user_agent)),
                api_key: Some(openapi::apis::configuration::ApiKey {
                    key: "Token=\"628b40e9955943b18c5ab4c7168e2e29\"".to_string(),
                    prefix: Some("Emby".to_string()),
                }),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub async fn load_server_data(self: &mut Self) -> Result<(), ServerFnError> {
        if self.metadata.is_none() {
            let server_name = match self.get_server_name().await {
                Ok(name) => name,
                Err(e) => return Err(e),
            };

            let admin_user_id = match self.get_admin_user().await {
                Ok(id) => id,
                Err(e) => return Err(e),
            };

            self.metadata = Some(JellyfinServerMetadata {
                server_name,
                admin_user_id,
            });
        }

        return Ok(());
    }

    async fn get_server_name(self: &Self) -> Result<String, ServerFnError> {
        let result = openapi::apis::system_api::get_public_system_info(&self.conf).await;

        match result {
            Ok(openapi::apis::ResponseContent {
                status: _,
                content: _,
                entity: Some(openapi::apis::system_api::GetPublicSystemInfoSuccess::Status200(data)),
            }) => {
                let name = data.server_name.unwrap().unwrap();
                return Ok(name);
            }
            Ok(openapi::apis::ResponseContent {
                status: _,
                content: _,
                entity: _,
            }) => {
                return Err(ServerFnError::ServerError(
                    "Could not connect to jellyfin".to_string(),
                ));
            }
            Err(err) => {
                eprintln!("Error sending message: {}", err);

                return Err(ServerFnError::ServerError(
                    "Could not connect to jellyfin".to_string(),
                ));
            }
        }
    }

    async fn get_admin_user(self: &Self) -> Result<uuid::Uuid, ServerFnError> {
        let result = openapi::apis::user_api::get_users(
            &self.conf,
            openapi::apis::user_api::GetUsersParams {
                ..Default::default()
            },
        )
        .await;

        match result {
            Ok(openapi::apis::ResponseContent {
                status: _,
                content: _,
                entity: Some(openapi::apis::user_api::GetUsersSuccess::Status200(data)),
            }) => {
                for user in data {
                    if user.policy.unwrap().unwrap().is_administrator.unwrap() {
                        return Ok(user.id.unwrap());
                    }
                }
                return Err(ServerFnError::ServerError(
                    "There is no admin user to use for the transfer.".to_string(),
                ));
            }
            Ok(openapi::apis::ResponseContent {
                status,
                content: _,
                entity: _,
            }) => {
                println!("Got data non 200! {}", status);
                return Err(ServerFnError::ServerError(
                    "Could not connect to jellyfin".to_string(),
                ));
            }
            Err(err) => {
                eprintln!("Error sending message: {}", err);

                return Err(ServerFnError::ServerError(
                    "Could not connect to jellyfin".to_string(),
                ));
            }
        }
    }

    async fn get_movies(self: &Self) -> Result<uuid::Uuid, ServerFnError>{
        todo!()
    }
}
