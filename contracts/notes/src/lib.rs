#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Item {
    id: u64,
    name: String,
    quantity: u32,
    price: u32,
    expired_at: u64, // timestamp
}

const ITEM_DATA: Symbol = symbol_short!("ITEM_DATA");

#[contract]
pub struct SmartInventory;

#[contractimpl]
impl SmartInventory {

    pub fn get_items(env: Env) -> Vec<Item> {
        env.storage().instance().get(&ITEM_DATA).unwrap_or(Vec::new(&env))
    }

    pub fn add_item(env: Env, name: String, quantity: u32, price: u32, expired_at: u64) -> String {
        let mut items: Vec<Item> = env.storage().instance().get(&ITEM_DATA).unwrap_or(Vec::new(&env));

        let item = Item {
            id: env.prng().gen::<u64>(),
            name,
            quantity,
            price,
            expired_at,
        };

        items.push_back(item);
        env.storage().instance().set(&ITEM_DATA, &items);

        String::from_str(&env, "Item berhasil ditambahkan")
    }

    pub fn delete_item(env: Env, id: u64) -> String {
        let mut items: Vec<Item> = env.storage().instance().get(&ITEM_DATA).unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            if items.get(i).unwrap().id == id {
                items.remove(i);
                env.storage().instance().set(&ITEM_DATA, &items);
                return String::from_str(&env, "Item berhasil dihapus");
            }
        }

        String::from_str(&env, "Item tidak ditemukan")
    }
        pub fn update_item(
        env: Env,
        id: u64,
        name: String,
        quantity: u32,
        price: u32,
        expired_at: u64
    ) -> String {
        let mut items: Vec<Item> = env.storage().instance().get(&ITEM_DATA).unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            let mut item = items.get(i).unwrap();

            if item.id == id {
                item.name = name;
                item.quantity = quantity;
                item.price = price;
                item.expired_at = expired_at;

                items.set(i, item);
                env.storage().instance().set(&ITEM_DATA, &items);

                return String::from_str(&env, "Item updated successfully");
            }
        }

        String::from_str(&env, "Item not found")
    }

    // 🔥 fitur unik
    pub fn get_expiring_items(env: Env, current_time: u64) -> Vec<Item> {
        let items: Vec<Item> = env.storage().instance().get(&ITEM_DATA).unwrap_or(Vec::new(&env));
        let mut result = Vec::new(&env);

        for i in 0..items.len() {
            let item = items.get(i).unwrap();

            // jika kurang dari 3 hari (259200 detik)
            if item.expired_at <= current_time + 259200 {
                result.push_back(item);
            }
        }

        result
    }
}

mod test;