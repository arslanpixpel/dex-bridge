use aws_sdk_secretsmanager::Client;
use concordium_rust_sdk::types::WalletAccount;
use ethers::signers::LocalWallet;

/// Load concordium keys from the secret manager.
/// The configuration is loaded from the environment variables
/// - `AWS_ACCESS_KEY_ID`
/// - `AWS_SECRET_ACCESS_KEY` with fallback to `SECRET_ACCESS_KEY`
/// - `AWS_SESSION_TOKEN`
/// - `AWS_REGION`
#[cfg(feature = "kms")]
pub async fn get_concordium_keys_aws(secret_name: &str) -> anyhow::Result<WalletAccount> {
    log::debug!("Loading Concordium keys from AWS secret manager!");
    let shared_config = aws_config::load_from_env().await;

    let client = Client::new(&shared_config);

    let resp = client
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await?;
    let Some(raw_secret) = resp.secret_string() else {
        anyhow::bail!("Secret {secret_name} was not present")
    };
    let acc = WalletAccount::from_json_str(raw_secret)?;
    Ok(acc)
}

#[cfg(feature = "default")]
pub async fn get_concordium_keys_aws(secret_name: &str) -> anyhow::Result<WalletAccount> {
    log::debug!("Loading Concordium keys from ENV!");

    let acc = WalletAccount::from_json_str(
        &std::env::var(format!("CONCORDIUM_KEY_{secret_name}")).unwrap(),
    )?;
    Ok(acc)
}

/// Load Ethereum keys from the secret manager.
/// The configuration is loaded from the environment variables
/// - `AWS_ACCESS_KEY_ID`
/// - `AWS_SECRET_ACCESS_KEY` with fallback to `SECRET_ACCESS_KEY`
/// - `AWS_SESSION_TOKEN`
/// - `AWS_REGION`
#[cfg(feature = "kms")]
pub async fn get_ethereum_keys_aws(secret_name: &str) -> anyhow::Result<LocalWallet> {
    log::debug!("Loading Ethereum keys from AWS secret manager!");
    let shared_config = aws_config::load_from_env().await;

    let client = Client::new(&shared_config);

    let resp = client
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await?;
    let Some(raw_secret) = resp.secret_string() else {
        anyhow::bail!("Secret {secret_name} was not present")
    };
    let lw = raw_secret.parse()?;
    Ok(lw)
}

#[cfg(feature = "default")]
pub async fn get_ethereum_keys_aws(secret_name: &str) -> anyhow::Result<LocalWallet> {
    log::debug!("Loading Ethereum keys from ENV!");

    let lw = std::env::var(format!("CONCORDIUM_KEY_{secret_name}"))?.parse()?;
    Ok(lw)
}
