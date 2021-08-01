use testcontainers::*;

#[test]
fn trufflesuite_ganachecli_listaccounts() {
    let _ = pretty_env_logger::try_init();
    let docker = clients::Cli::default();
    let node = docker.run(images::trufflesuite_ganachecli::GanacheCli::default());
    let host_port = node.get_host_port(8545).unwrap();

    let response = reqwest::blocking::Client::new()
        .post(&format!("http://localhost:{}", host_port))
        .body(
            json::object! {
                "jsonrpc" => "2.0",
                "method" => "net_version",
                "params" => json::array![],
                "id" => 1
            }
            .dump(),
        )
        .header("content-type", "application/json")
        .send()
        .unwrap();

    let response = response.text().unwrap();
    let response = json::parse(&response).unwrap();

    assert_eq!(response["result"], "42");
}
