# Thorfinn: The Nostr Extension for Data Vending Machine use

A Nostr signer extension compatible with NIP-07, optimised for DVM ([NIP-90 - Data Vending Machine](https://nips.nostr.com/90)) use.

## Features

- Generate a new keypair
- Sign events with the generated keypair
- Store the keypair in local storage
- Load the keypair from local storage
- Sign events with the loaded keypair
- Submit DVM Job Requests
- Monitor DVM Job Requests
- Fetch DVM Job Results
- Dashboard for managing jobs
- Settings for customizing the extension

## Installation

1. Clone the repository
2. Run `npm install`
3. Run `npm run build`
4. Load the extension in Chrome by navigating to `chrome://extensions/`, enabling "Developer mode", and clicking "Load unpacked".

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
