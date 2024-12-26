## Solana Marketplace Program

A marketplace program where : 
- admin can initialize marketplace
- user can list their NFTs, 
- users can delist their listed NFTs,
- users can purchase other NFTs

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

- Initialize: initilaizes the marketplace with state with a specific name and related accounts by admin
