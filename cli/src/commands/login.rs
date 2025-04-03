use crate::handle_operation_error;
use dialoguer::{Input, Password, Select};
use sindri::{client::SindriClient, TeamDetail};

pub fn login(client: &SindriClient, username: Option<String>, password: Option<String>) {
    println!("{}", console::style("Logging in...").bold());

    let username = match username {
        Some(u) => u,
        None => Input::new()
            .with_prompt("Username")
            .interact_text()
            .unwrap_or_else(|e| handle_operation_error("Login", &e.to_string())),
    };

    let password = match password {
        Some(p) => p,
        None => Password::new()
            .with_prompt("Password")
            .interact()
            .unwrap_or_else(|e| handle_operation_error("Login", &e.to_string())),
    };

    // Generate an initial JWT token for team retrieval
    let rt = tokio::runtime::Runtime::new().unwrap();
    let token = match rt.block_on(client.jwt_token_generate(&username, &password)) {
        Ok(token) => token,
        Err(e) => handle_operation_error("Login", &e.to_string()),
    };

    // Collect list of teams for the user
    let teams = match rt.block_on(client.teams_jwt_auth(&token)) {
        Ok(teams) => teams,
        Err(e) => handle_operation_error("Login", &e.to_string()),
    };
    if teams.is_empty() {
        handle_operation_error("Login", "No teams found for this user");
    }

    // Let user select a team
    let team_names: Vec<String> = teams.iter().map(|t: &TeamDetail| t.slug.clone()).collect();
    let selection = Select::new()
        .with_prompt("Select a team to generate an API key for")
        .items(&team_names)
        .interact()
        .unwrap_or_else(|e| handle_operation_error("Login", &e.to_string()));
    let selected_team = &teams[selection];

    // Generate API key for selected team
    let api_key = match rt.block_on(client.api_key_select_team(
        &username,
        &password,
        &selected_team.name,
        &selected_team.id.to_string(),
    )) {
        Ok(key) => key,
        Err(e) => handle_operation_error("Login", &e.to_string()),
    };

    println!(
        "\n{}",
        console::style("API key generated successfully!")
            .green()
            .bold()
    );
    println!(
        "{}",
        console::style("Please copy this key as it will only be shown once:").bold()
    );
    println!("{}", console::style(api_key).cyan());
    println!("\n{}", console::style("You can now use this key with the --api-key flag or set `SINDRI_API_KEY` in your environment variables.").dim());
}
