use testcontainers::*;

#[tokio::test]
async fn trufflesuite_ganachecli_listaccounts() -> web3::Result<()> {
    let _ = pretty_env_logger::try_init();
    let docker = clients::Cli::default();
    let node = docker.run(images::trufflesuite_ganachecli::GanacheCli::default());
    let host_port = node.get_host_port(8545).unwrap();

    let address = format!("http://localhost:{}", host_port);

    let transport = web3::transports::Http::new(&address)?;
    let web3 = web3::Web3::new(transport);

    println!("Calling accounts.");
    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
    }

    Ok(())
}
