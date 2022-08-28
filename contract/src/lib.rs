/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};
use near_sdk::{env, AccountId, BorshStorageKey, PanicOnDefault};
use near_sdk::collections::{LookupMap};
use near_sdk::serde::{Deserialize, Serialize};

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

// Declare the type of task
pub type TaskId = u32;
// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    task_id_by_accounts: LookupMap<AccountId, Vec<u32>>,
    task_by_id: LookupMap<u32,Task>,
    // message: String
}

// Define tasks todo list 
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate="near_sdk::serde")]
#[derive(Debug)]
pub struct Task {
    id: u32,
    content: String,
    is_complete: bool,
    owner_id: AccountId
}

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
enum StorageKey {
    TaskByIdKey,
    TaskByAccountIdKey,
}

// Define the default, which automatically initializes the contract
// impl Default for Contract{
//     fn default() -> Self{
//         Self{
//             owner_id,
//             task_id_by_accounts: LookupMap::new(StorageKey::TaskByAccountIdKey), 
//             task_by_id: LookupMap::new(StorageKey::TaskByIdKey),
//             // message: DEFAULT_MESSAGE.to_string()
//         }
//     }
// }

// Implement the contract structure
#[near_bindgen]
impl Contract {

     #[init]
    pub fn new(owner_id: AccountId) -> Self{
        Self{
            owner_id,
            task_id_by_accounts: LookupMap::new(StorageKey::TaskByAccountIdKey), 
            task_by_id: LookupMap::new(StorageKey::TaskByIdKey),
            // message: DEFAULT_MESSAGE.to_string()
        }
    }
    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    // pub fn get_greeting(&self) -> String {
    //     return self.message.clone();
    // }

    // // Public method - accepts a greeting, such as "howdy", and records it
    // pub fn set_greeting(&mut self, message: String) {
    //     // Use env::log to record logs permanently to the blockchain!
    //     log!("Saving greeting 12{}", message);
    //     self.message = message;
    // }

    // Update task for the owner
    // pub fn update_task(&mut self, task_id: u32) {
    //     let task: Task = Task { 
    //         task_id: task_id.clone(), 
    //         amount: order_amount.0, 
    //         received_amount: env::attached_deposit(), 
    //         is_completed: true, 
    //         is_refund: false, 
    //         created_at: env::block_timestamp()
    //     };
    //     self.task_id_by_accounts.insert(&task_id, task_id)
    // }

    // create new task
    pub fn create_task(&mut self, id: u32, content: String) {
        let task : Task = Task {
            id: id,
            content: content.clone(),
            is_complete: false,
            owner_id: self.owner_id.clone(),
        };
        self.task_by_id.insert(&task.id, &task);
        let mut taskAccount : Option<Vec<u32>> = self.task_id_by_accounts.get(&self.owner_id);
        // .unwrap().push(task.id);
        if taskAccount.is_none() {
            // taskAccount = Vec::new();
            let mut id = Vec::new();
            id.push(task.id);
            taskAccount.insert(id);
        }
        else {
            taskAccount.unwrap().push(task.id);
        }
    }

    // Get task
    pub fn get_task(&self, task_id: u32) -> Task {
        self.task_by_id.get(&task_id).expect("Task not found")
    }

    // Get all of task by accountId
    pub fn get_task_by_accounts(&self, account_id: AccountId) {
        for task_id in self.task_id_by_accounts.get(&account_id).unwrap() {
            let task : Task = Contract::get_task(self, task_id);
            log!("Task `{:?}`", task);
        }
    }
}


/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            contract.get_greeting(),
            "Hello".to_string()
        );
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            contract.get_greeting(),
            "howdy".to_string()
        );
    }
}
