use color_eyre::Result;
use ethers::prelude::*;
use ethers_flashbots::FlashbotsMiddleware;
use fractional::{TokenVault, VaultOpts, TOKEN_VAULT};
use std::sync::Arc;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = VaultOpts::from_args();

    let provider = opts.eth.provider()?;

    if let Some(bribe) = opts.flashbots.bribe {
        println!(
            "Using Flashbots. Bribe {:?}. Bribe Receiver {:?}",
            bribe, opts.flashbots.bribe_receiver
        );

        let bundle_signer = LocalWallet::new(&mut ethers::core::rand::thread_rng());
        let provider = FlashbotsMiddleware::new(
            provider,
            url::Url::parse("https://relay.flashbots.net")?,
            bundle_signer,
        );

        bid(provider, opts).await?;
    } else {
        bid(provider, opts).await?;
    };

    Ok(())
}

async fn bid<M: Middleware + 'static>(provider: M, opts: VaultOpts) -> Result<()> {
    let chain_id = provider.get_chainid().await?.as_u64();
    if let Some(signer) = opts.fireblocks.signer(chain_id).await? {
        // will just sign transactions with the fireblocks signer, and submit them
        // via Flashbots as usual
        let provider = Arc::new(SignerMiddleware::new(provider, signer));
        let fractional = TokenVault::new(*TOKEN_VAULT, provider.clone());

        // TODO: We should be able to erase the type of the provider so they can
        // be easily composed.
        let call = fractional.bid().value(opts.amount);
        let res = call.send().await?;
        println!("Submitted tx: {:?}", *res);
        let receipt = res.await?;
        println!("Got receipt: {:?}", receipt);
    } else {
        let signer = opts.eth.signer()?.with_chain_id(chain_id);
        let provider = Arc::new(SignerMiddleware::new(provider, signer));
        let fractional = TokenVault::new(*TOKEN_VAULT, provider.clone());

        let call = fractional.bid().value(opts.amount);
        let res = call.send().await?;
        println!("Submitted tx: {:?}", *res);
        let receipt = res.await?;
        println!("Got receipt: {:?}", receipt);
    };

    Ok(())
}
