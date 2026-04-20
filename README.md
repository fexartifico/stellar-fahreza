# 📦 Smart Inventory & Expiry Tracker (Soroban Smart Contract)

![Rust](https://img.shields.io/badge/Rust-Programming-orange)
![Soroban](https://img.shields.io/badge/Soroban-SmartContract-blue)
![Status](https://img.shields.io/badge/Status-Active-green)

---

## 🚀 Overview

Smart Inventory & Expiry Tracker is a decentralized inventory management system built using Soroban (Stellar Smart Contracts).

Unlike traditional inventory systems, this project introduces expiry tracking logic, allowing users to monitor items that are close to expiration and take action before they go to waste.

This project is designed for real-world use cases such as:

* Household inventory management
* Small shops / minimarkets
* Food storage tracking

---

## ✨ Features

* Add new items (name, quantity, price, expiry date)
* Retrieve all stored items
* Delete items by ID
* Update item data (full update support)
* Detect items that are about to expire (within 3 days)

---

## 📸 Demo

Example of expiring items output:

```json
[
  {
    "id": 1,
    "name": "Milk",
    "quantity": 2,
    "price": 15000,
    "expired_at": 1715000000
  }
]
```

---

## ⚙️ Installation

1. Clone this repository:

```bash
git clone https://github.com/your-username/smart-inventory.git
cd smart-inventory
```

2. Install Soroban CLI (if not installed)

3. Build the contract:

```bash
cargo build --target wasm32-unknown-unknown --release
```

---

## ▶️ Usage

You can interact with the contract using Soroban CLI:

### Add Item

```bash
add_item("Milk", 2, 15000, 1715000000)
```

### Get All Items

```bash
get_items()
```

### Get Expiring Items

```bash
get_expiring_items(current_time)
```

### Update Item

```bash
update_item(id, name, quantity, price, expired_at)
```

### Delete Item

```bash
delete_item(id)
```

---

## 🧪 Example

Input:

```bash
add_item("Bread", 5, 10000, 1715000000)
```

Output:

```bash
"Item successfully added"
```

---

## 🏗️ Data Structure

```rust
pub struct Item {
    id: u64,
    name: String,
    quantity: u32,
    price: u32,
    expired_at: u64,
}
```

---

## 🛠️ Tech Stack

* Rust
* Soroban SDK
* Stellar Blockchain

---

## 📁 Project Structure

```
├── src
│   ├── lib.rs
│   ├── test.rs
```

---

## ⚠️ Limitations

* No authentication or user roles
* No frontend interface (CLI-based interaction only)
* Data stored fully on-chain
* No real-time notification system

---

## 🚀 Future Improvements

* Role-based access control (admin/user)
* Frontend integration (React / Next.js)
* Notification system for expiring items
* Inventory analytics dashboard
* Stock in/out transaction history

---

## 🎯 Why This Project?

This project demonstrates:

* Smart contract development using Soroban
* Implementation of business logic beyond CRUD
* Real-world problem solving using blockchain

---

## 👨‍💻 Author

Reza
Informatics Engineering Student – Politeknik Negeri Jakarta

---

## ⭐ Support

If you find this project useful, feel free to give it a star on GitHub!

ID SmartContract :  CDC3ZZJA5DERB7VQL55XBRLLS5REAU26YYZ7U5ZHDLEQL6LTMKAO5ZTM
