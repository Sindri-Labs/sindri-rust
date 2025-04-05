//! Methods used by the CLI to authenticate with user/pass before generating an API key
//!
//! These methods should be used with caution outside of the CLI since there are
//! no intentional checks in place to ensure the token is valid or has not expired.

use sindri_openapi::{
    apis::{
        authorization_api::apikey_generate, configuration::Configuration,
        token_api::jwt_token_generate, user_me_with_jwt_auth,
    },
    models::{ObtainApikeyInput, TeamDetail, TokenObtainPairInputSchema},
};

use crate::client::SindriClient;

impl SindriClient {
    /// Generate and return a JWT token from a username and password
    ///
    /// A nonstandard base URL is passed through the SindriClient
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

    /// Get the teams for a user
    ///
    /// Any API key attached to the SindriClient is replaced with
    /// the input JWT token.
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

    /// Generate an API key from a username and password
    ///
    /// A `Sindri-Team-Id` header is passed to the API to select the
    /// team for which the API key will be generated.
    pub async fn api_key_select_team(
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

#[cfg(test)]
mod tests {
    use super::*;
    use sindri_openapi::models::{ApiKeyResponse, TokenObtainPairOutputSchema, UserMeResponse};
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    async fn mock_auth_server() -> MockServer {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/token/pair"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(TokenObtainPairOutputSchema {
                    access: "test_jwt_token".to_string(),
                    refresh: "test_refresh_token".to_string(),
                    ..Default::default()
                }),
            )
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/api/v1/user/me"))
            .respond_with(ResponseTemplate::new(200).set_body_json(UserMeResponse {
                teams: vec![TeamDetail {
                    id: 1,
                    name: "Team One".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            }))
            .mount(&mock_server)
            .await;

        Mock::given(method("POST"))
            .and(path("/api/apikey/generate"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ApiKeyResponse {
                api_key: Some("test_api_key_123".to_string()),
                ..Default::default()
            }))
            .mount(&mock_server)
            .await;

        mock_server
    }

    #[tokio::test]
    async fn test_jwt_token_generate() {
        let mock_server = mock_auth_server().await;

        let mut client = SindriClient::default();
        client.config.base_path = mock_server.uri().to_string();

        let token = client
            .jwt_token_generate("test_user", "test_pass")
            .await
            .unwrap();

        assert_eq!(token, "test_jwt_token");
    }

    #[tokio::test]
    async fn test_teams_jwt_auth() {
        let mock_server = mock_auth_server().await;

        let mut client = SindriClient::default();
        client.config.base_path = mock_server.uri().to_string();

        let teams = client.teams_jwt_auth("test_jwt_token").await.unwrap();

        assert_eq!(teams.len(), 1);
        assert_eq!(teams[0].id, 1);
    }

    #[tokio::test]
    async fn test_api_key_select_team() {
        let mock_server = mock_auth_server().await;

        let mut client = SindriClient::default();
        client.config.base_path = mock_server.uri().to_string();

        let api_key = client
            .api_key_select_team("test_user", "test_pass", "test_key", "team1")
            .await
            .unwrap();

        assert_eq!(api_key, "test_api_key_123");
    }
}
