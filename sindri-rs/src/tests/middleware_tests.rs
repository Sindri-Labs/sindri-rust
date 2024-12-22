


#[tokio::test]
async fn test_new_client_default_headers() {
    let client = SindriClient::new(None);
    let config = client.config();

    // Example code from openapi/src/apis/authorization_api.rs
    // This code is used in nearly all methods. However it isn't modularized by the codegen
    // So we can't call a method from that package.
    // let local_var_configuration = client.config();

    // let local_var_client = &local_var_configuration.client;

    // // TODO: change so it's not hitting api
    // let local_var_uri_str = format!("{}/api/v1/sindri-manifest-schema.json", local_var_configuration.base_path);
    // let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    // if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
    //     local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    // }
    // if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
    //     local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    // };
    // if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
    //     local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    // };

    // let local_var_req = local_var_req_builder.build().unwrap();
    // println!("{:?}", local_var_req.headers());
    // let local_var_resp = local_var_client.execute(local_var_req).await.unwrap();
    // let headers = local_var_resp.
    // .request().headers();
    // println!("{:?}", headers);

    // println!("{:?}",headers);

    assert_eq!(headers.get("User-Agent").unwrap(), "OpenAPI-Generator/v1.14.5/rust");
    assert!(headers.get("Authorization").unwrap().to_str().unwrap().starts_with("Bearer "));
    assert!(headers.contains_key("sindri-client"));

}