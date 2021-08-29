# fractional-rs

CLI & Utilities for [fractional.art](https://fractional.art)

## CLI Usage

The CLI uses Flashbots' relay to submit the transactions. No bribe is required as
you pay via the fee. It also supports [Fireblocks](fireblocks.com) API (requires vaults
with the RAW feature on).

```
$ target/debug/fractional -h
fractional 0.1.0

USAGE:
    fractional [OPTIONS] <vault> <amount> --eth.url <url>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --fireblocks.key <api-key>                     Your fireblocks API key [env: FIREBLOCKS_API_KEY=]
        --flashbots.bribe <bribe>                      The amount to be sent to the miner
        --flashbots.bribe_receiver <bribe-receiver>
            The address that will receive the bribe. Ideally it should be a smart contract with a block.coinbase
            transfer
        --eth.hd_index <index>                         your index in the standard hd path [default: 0]
        --eth.mnemonic <mnemonic-path>                 Path to your mnemonic file
        --eth.private_key <private-key>                Your private key string
        --fireblocks.secret <secret>
            Path to your fireblocks.key file generated during api account creation [env: FIREBLOCKS_API_SECRET_PATH=]

    -u, --eth.url <url>                                The tracing / archival node's URL
        --fireblocks.vault <vault-id>
            The fireblocks vault which will be used for authorizing transactions [env: FIREBLOCKS_VAULT=]


ARGS:
    <vault>     the fractional vault you're calling
    <amount>    your bid (in wei)
```

## Deployments

### Mainnet
[Vault Factory](https://etherscan.io/address/0x85aa7f78bdb2de8f3e0c0010d99ad5853ffcfc63)

[Token Vault](https://etherscan.io/address/0x7b0fce54574d9746414d11367f54c9ab94e53dca)

[Settings](https://etherscan.io/address/0xE0FC79183a22106229B84ECDd55cA017A07eddCa)

[Index ERC721 Factory](https://etherscan.io/address/0xde771104c0c44123d22d39bb716339cd0c3333a1)

### Rinkeby

[Vault Factory](https://rinkeby.etherscan.io/address/0x458556c097251f52ca89cB81316B4113aC734BD1)

[Token Vault](https://rinkeby.etherscan.io/address/0x825f25f908db46daEA42bd536d25f8633667f62b)

[Settings](https://rinkeby.etherscan.io/address/0x1C0857f8642D704ecB213A752A3f68E51913A779)

[Index ERC721 Factory](https://rinkeby.etherscan.io/address/0xee727b734aC43fc391b67caFd18e5DD4Dc939668)
