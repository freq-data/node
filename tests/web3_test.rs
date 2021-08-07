use testcontainers::clients::Cli;
use testcontainers::images::trufflesuite_ganachecli::GanacheCli;
use testcontainers::*;
use web3::transports::Http;
use web3::types::BlockId;
use web3::types::BlockNumber;
use web3::types::U64;
use web3::Web3;

fn setup_docker(docker: &Cli) -> Container<'_, Cli, GanacheCli> {
    docker.run(images::trufflesuite_ganachecli::GanacheCli::default())
}

fn setup_web3(node: &Container<'_, Cli, GanacheCli>) -> web3::Result<Web3<Http>> {
    let host_port = node.get_host_port(8545).unwrap();
    let address = format!("http://localhost:{}", host_port);

    let transport = web3::transports::Http::new(&address)?;
    Ok(web3::Web3::new(transport))
}

#[tokio::test]
async fn list_accounts() -> web3::Result<()> {
    let docker = clients::Cli::default();
    let node = setup_docker(&docker);
    let web3 = setup_web3(&node)?;

    println!("Calling accounts.");
    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);

    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
    }
    //assert_eq!(accounts.len(), 8);

    Ok(())
}

#[tokio::test]
async fn list_blocks() -> web3::Result<()> {
    let docker = clients::Cli::default();
    let node = setup_docker(&docker);
    let web3 = setup_web3(&node)?;

    let mut block_number = web3.eth().block_number().await?;

    println!("Current block is {}", block_number);

    println!("Calling block.");

    let mut it = U64::from(0);

    while it <= block_number {
        let block = web3
            .eth()
            .block(BlockId::Number(BlockNumber::from(it)))
            .await?;
        println!("Block {} contains {:?}", it, block);
        it = it + 1
    }
    Ok(())
}
