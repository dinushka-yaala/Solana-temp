# Solana Dev Notes

## Start Local Net for Testing

### Start the local net in the current directory
```sh
solana-test-validator
```

### Create a wallet if there is none
```sh
solana-keygen grind --ignore-case --starts-with QN:1
```

### Set the new wallet
```sh
solana config set --url localhost --keypair <PATH_TO_WALLET>
```
Example:
```sh
solana config set --url localhost --keypair ./QN1seG8d1zZb5p5Do5gi5Y5Y5Y5Y5Y5Y5Y5Y5Y5Y.json
```

## Get SOL
```sh
solana airdrop 100
```

## Transfer SOL
```sh
solana transfer DEmoM52P1ci8Y6YQJbZVZjxUa4Arbb8pAjaPmg7nVda5 10 --allow-unfunded-recipient
```

## Test Anchor Program in the Local Net
```sh
anchor test --skip-local-validator
