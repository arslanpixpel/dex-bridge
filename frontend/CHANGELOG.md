## 1.0.2

### Fixed
- An issue where the wrong information would sometimes be shown on the transfer status page.

## 1.0.1

### Added
- A status on the transaction history page specifically for withdrawals which are ready for approval.

### Fixed
- A bug where a much larger estimate than necessary was used for calculating a max for ETH deposits.

## 1.0.0

### Added
- Warns users if the required wallet is not available when trying to connect.

### Changed
- Notifies user of pending withdraw approval rather than automatically routing to approval page.

### Fixed
- When selecting "max" for ETH deposits, an estimate of the required gas fee is taken into account.

## 0.1.1

### Added
- Environment variables for configuring access to a node's grpc interface. This is a temporary requirement, as the functionality this is needed for will be exposed by the concordium browser wallet at a later point in time.
- Application version visible on page (next to concordium logo)

### Changed
- For withdrawals, the expected time of the next merkle root, i.e. when the next round of withdrawals will be ready for approval, is now displayed instead of statically showing 10 minute processing time.
- Provide a full overview of the transactions (and corresponding fees) required to execute a full withdrawal/desposit.
- Automatically connect to first ethereum account which has already been approved for connection in MetaMask.
- Submitted transactions are now stored in the browser's storage upon submission, to allow users to see them instantly in the transaction history.

### Fixed
- Handle incorrect networks selected in both ethereum and concordium wallets appropriately.
- A number of minor bugs

## 0.1.0

Initial version.
