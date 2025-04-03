//! Methods used by the CLI to authenticate with user/pass before generating an API key
//! 
//! These methods should be used with caution outside of the CLI since there are
//! no checks in place to ensure the token is valid or has not expired.

use sindri_openapi::{
    apis::{authorization_api::apikey_generate, configuration::Configuration, token_api::jwt_token_generate, user_me_with_jwt_auth},
    models::{ObtainApikeyInput, TeamDetail, TokenObtainPairInputSchema},
};

use crate::client::SindriClient;

impl SindriClient {
    pub async fn jwt_token_generate(
        &self,
        username: &str,
        password: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let token = jwt_token_generate(
            &self.config,
            TokenObtainPairInputSchema {
                username: username.to_string(),
                password: password.to_string(),
            },
        )
        .await?;

        Ok(token.access)
    }

    pub async fn teams_jwt_auth(
        &self,
        token: &str,
    ) -> Result<Vec<TeamDetail>, Box<dyn std::error::Error>> {
        let config = Configuration {
            base_path: self.config.base_path.clone(),
            client: self.config.client.clone(),
            bearer_access_token: Some(token.to_string()),
            ..Default::default()
        };
        let user = user_me_with_jwt_auth(&config).await?;

        Ok(user.teams)
    }

    pub async fn api_key_from_jwt_team(
        &self,
        username: &str,
        password: &str,
        key_name: &str,
        team_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let api_key = apikey_generate(
            &self.config,
            ObtainApikeyInput {
                username: username.to_string(),
                password: password.to_string(),
                name: Some(key_name.to_string()),
            },
            Some(team_id),
        )
        .await?;

        if let Some(api_key) = api_key.api_key {
            Ok(api_key)
        } else {
            Err("Failed to generate API key".into())
        }
    }


}
