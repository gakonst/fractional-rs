use color_eyre::Result;
use ethers::prelude::*;
use once_cell::sync::Lazy;

pub static TOKEN_VAULT: Lazy<Address> = Lazy::new(|| {
    "0x7b0fce54574d9746414d11367f54c9ab94e53dca"
        .parse()
        .unwrap()
});

use ethers::contract::abigen;
abigen!(TokenVault, "./abis/token_vault.json");

use ethers_structopt::{EthereumOpts, FlashBotsOpts};
use structopt::StructOpt;

#[derive(StructOpt, Clone)]
pub struct VaultOpts {
    #[structopt(long, short, help = "the fractional vault you're calling")]
    pub vault: Address,

    #[structopt(long, short, help = "your bid (in wei)")]
    pub amount: U256,

    #[structopt(flatten)]
    pub eth: EthereumOpts,

    #[structopt(flatten)]
    pub flashbots: FlashBotsOpts,

    #[structopt(flatten)]
    pub fireblocks: FireblocksOpts,
}

#[derive(StructOpt, Clone)]
pub struct FireblocksOpts {
    #[structopt(
        long = "fireblocks.secret",
        help = "Path to your fireblocks.key file generated during api account creation",
        env = "FIREBLOCKS_API_SECRET_PATH"
    )]
    secret: Option<String>,

    #[structopt(
        long = "fireblocks.key",
        help = "Your fireblocks API key",
        env = "FIREBLOCKS_API_KEY",
        requires = "secret"
    )]
    api_key: Option<String>,

    #[structopt(
        long = "fireblocks.vault",
        help = "The fireblocks vault which will be used for authorizing transactions",
        env = "FIREBLOCKS_VAULT",
        requires = "secret"
    )]
    vault_id: Option<String>,
}

use ethers_fireblocks::{Config, FireblocksSigner};
impl FireblocksOpts {
    pub async fn signer(&self, chain_id: u64) -> Result<Option<FireblocksSigner>> {
        let (secret, api_key, vault_id) = match (&self.secret, &self.api_key, &self.vault_id) {
            (Some(a), Some(b), Some(c)) => (a, b, c),
            (None, None, None) => return Ok(None),
            _ => unreachable!("Did you set all Fireblocks options?"),
        };
        let cfg = Config::new(secret, &api_key, &vault_id, chain_id)?;
        let signer = FireblocksSigner::new(cfg).await;
        Ok(Some(signer))
    }
}
