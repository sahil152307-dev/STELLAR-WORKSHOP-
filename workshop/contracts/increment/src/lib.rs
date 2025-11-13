#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};

#[contract]
pub struct Contract;

// This is a sample contract. Replace this placeholder with your own contract logic.
// A corresponding test example is available in `test.rs`.
//
// For comprehensive examples, visit <https://github.com/stellar/soroban-examples>.
// The repository includes use cases for the Stellar ecosystem, such as data storage on
// the blockchain, token swaps, liquidity pools, and more.
//
// Refer to the official documentation:
// <https://developers.stellar.org/docs/build/smart-contracts/overview>.
#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}

mod test;
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, Address, symbol_short};

// Structure to track game asset details
#[contracttype]
#[derive(Clone)]
pub struct GameAsset {
    pub asset_id: u64,
    pub asset_name: String,
    pub owner: Address,
    pub price: u64,
    pub is_listed: bool,
}

// Counter for generating unique asset IDs
const ASSET_COUNT: Symbol = symbol_short!("A_COUNT");

// Mapping asset_id to GameAsset
#[contracttype]
pub enum AssetBook {
    Asset(u64)
}

#[contract]
pub struct GameTokenContract;

#[contractimpl]
impl GameTokenContract {

    // Function 1: Mint a new in-game asset as a token
    pub fn mint_asset(env: Env, owner: Address, asset_name: String, price: u64) -> u64 {
        // Require owner authorization
        owner.require_auth();
        
        // Get and increment asset count
        let mut asset_count: u64 = env.storage().instance().get(&ASSET_COUNT).unwrap_or(0);
        asset_count += 1;

        // Create new game asset
        let new_asset = GameAsset {
            asset_id: asset_count,
            asset_name: asset_name.clone(),
            owner: owner.clone(),
            price,
            is_listed: false,
        };

        // Store the asset
        env.storage().instance().set(&AssetBook::Asset(asset_count), &new_asset);
        env.storage().instance().set(&ASSET_COUNT, &asset_count);
        
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Asset minted with ID: {}", asset_count);
        asset_count
    }

    // Function 2: List an asset for trade in the marketplace
    pub fn list_asset(env: Env, seller: Address, asset_id: u64, new_price: u64) {
        // Require seller authorization
        seller.require_auth();

        // Get the asset
        let mut asset = Self::view_asset(env.clone(), asset_id);

        // Verify ownership
        if asset.owner != seller {
            log!(&env, "Only the owner can list this asset");
            panic!("Not the asset owner");
        }

        // Update listing status and price
        asset.is_listed = true;
        asset.price = new_price;

        // Store updated asset
        env.storage().instance().set(&AssetBook::Asset(asset_id), &asset);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Asset ID: {} listed for sale at price: {}", asset_id, new_price);
    }

    // Function 3: Transfer ownership of an asset (trade)
    pub fn transfer_asset(env: Env, buyer: Address, asset_id: u64) {
        // Require buyer authorization
        buyer.require_auth();

        // Get the asset
        let mut asset = Self::view_asset(env.clone(), asset_id);

        // Check if asset is listed for sale
        if !asset.is_listed {
            log!(&env, "Asset is not listed for sale");
            panic!("Asset not for sale");
        }

        // Transfer ownership
        asset.owner = buyer.clone();
        asset.is_listed = false;

        // Store updated asset
        env.storage().instance().set(&AssetBook::Asset(asset_id), &asset);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Asset ID: {} transferred to new owner", asset_id);
    }

    // Function 4: View asset details
    pub fn view_asset(env: Env, asset_id: u64) -> GameAsset {
        let key = AssetBook::Asset(asset_id);
        
        env.storage().instance().get(&key).unwrap_or(GameAsset {
            asset_id: 0,
            asset_name: String::from_str(&env, "Not_Found"),
            owner: Address::from_string(&String::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF")),
            price: 0,
            is_listed: false,
        })
    }
}
