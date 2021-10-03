# Wrapped Casper ERC20

Implementation of ERC20 token, representing wrapped version of CSPR (native token of Casper Network).

## Wrapping Casper

Any operation that ends with this contract holding Wrapped Casper is prohibited.

1. `deposit` Casper in this contract to receive Wrapped Casper (WCSPR), which implements the ERC20 standard. WCSPR is interchangeable with Casper in a 1:1 basis.

2. `withdraw` Casper from this contract by unwrapping WCSPR from your wallet.

## The Friendly Hackathon: Start Building On Casper!

Project is participant of Open topic bounty
https://gitcoin.co/issue/casper-network/gitcoin-hackathon/26/100026594
