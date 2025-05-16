# Blind Auction, using Homomorphic Encryption

A blind auction is a type of auction where the bidders do not know the identity of the other bidders or the amount they are bidding

Implementation Architecture:

Bid Storage: 
- Contract stores encrypted bids without ability to decrypt
- Bidders do not know the identity of the other bidders or the amount they are bidding

Winner Determination: 
- Once auction ends, computations on encrypted data reveal winner

Bid Revelation: 
- Winner provides proof or decryption key to claim their win

-## Project Structure

```text

