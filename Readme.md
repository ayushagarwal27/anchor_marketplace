## Solana Marketplace Program

A marketplace program where : 
- admin can initialize marketplace
- maker can list their NFT, 
- maker can delist their listed NFT,
- taker can purchase listed NFT

---

### State

Marketplace

```rs
pub struct Marketplace {
    pub admin: Pubkey,
    pub fee: u64,
    pub treasury_bump: u8,
    pub rewards_bump: u8,
    pub bump: u8,
    #[max_len(32)]
    pub name: String,
}
```

Listing
```rs
pub struct Listing {
    pub maker: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub bump: u8,
}
```

--- 
### Instructions

- Init: initilaizes the marketplace with pda with a specific name and related accounts by admin
- List: initilaizes the listing pda, transfer NFT from user's NFT ATA to Vault owned by Listing pda
- Purchase: 
  - send price (sol) from taker to maker deducting marketplace fee, 
  - transfer fee from taker to treasury of marketplace
  - send nft from vault of marketplace to taker ATA
  - close marketplace vault
- Delist: 
  - send nft from marketplace vault to maker ata
  - close marketplace vault