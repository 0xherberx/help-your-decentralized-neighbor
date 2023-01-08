// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
//use near_sdk::env::random_seed;
use near_sdk::{env, log, Balance, Promise, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, require};
use near_sdk::json_types::{*};
use near_sdk::serde::{self, Serialize};
use near_sdk::collections::{Vector, UnorderedMap, UnorderedSet};


pub const STORAGE_COST: Balance = 1_000_000_000_000_000_000_000; //0.001 NEAR

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    ITEM,
}

// Create a struct DonationMessage to keep track of important information
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DonationMessage {
    donation_amount: u128,
    message: String,
}

impl DonationMessage {
    pub fn new(text_message: String) -> Self {
        Self {
            donation_amount: env::attached_deposit(),            
            message: text_message,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct DonationsByAccount {
    account_donations: Vector<DonationMessage>,
    total_donations_by_account: u128,
}

#[near_bindgen]
//#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    donations: UnorderedMap<AccountId, DonationsByAccount>,
    total_global_amount_donated: u128,
    registered_lottery_accounts: UnorderedSet<AccountId>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            donations: UnorderedMap::new(b"map-uid-1".to_vec()),
            total_global_amount_donated: 0,
            registered_lottery_accounts: UnorderedSet::new(b"set-uid-1".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(msg: String) -> Self {
        let donation_msg: DonationMessage = DonationMessage::new(msg);
        let mut acc_donations: Vector<DonationMessage> = Vector::new(b"a");
        acc_donations.push(&donation_msg);

        let donations_by_account: DonationsByAccount = DonationsByAccount {
            account_donations: acc_donations,
            total_donations_by_account: donation_msg.donation_amount,
        };
        let donor: AccountId = env::signer_account_id();
        // let donor: AccountId = env::predecessor_account_id();
        let mut donations_map: UnorderedMap<AccountId, DonationsByAccount> = UnorderedMap::new(b"map-uid-1".to_vec());
        donations_map.insert(&donor, &donations_by_account);

        Self {            
            donations: donations_map,
            total_global_amount_donated: donation_msg.donation_amount,
            registered_lottery_accounts: UnorderedSet::new(b"set-uid-1".to_vec()),
        }
    }

    pub fn get_total_global_amount_donated(&self) -> U128 {
        U128(self.total_global_amount_donated)
    }

    // Returns an array of last N messages. Paginates the messages using the from_index and limit parameters.
    pub fn get_account_donations(&self, donor: AccountId, from_index: usize, limit: usize) -> Vec<DonationMessage> {
        assert!(self.donations.get(&donor).is_some(), "Cannot find {} as donor", donor.to_string());
        // Collect the requested messages
        self.donations.get(&donor).unwrap().account_donations.iter().skip(from_index).take(limit).collect()
    }

    pub fn get_registered_lottery_accounts(&self) -> Vec<AccountId>{
        return self.registered_lottery_accounts.to_vec();
    }

    #[payable] // Public - People can attach money
    pub fn add_donation(&mut self, text_message: String) -> DonationMessage {
        let donation_msg: DonationMessage = DonationMessage::new(text_message);
        log!("donation_msg: {:?}", donation_msg);

        let donor: AccountId = env::signer_account_id();
        let donation_amount: Balance = donation_msg.donation_amount;

        let mut donated_so_far: Balance = if !self.donations.get(&donor).is_none() {
            self.donations.get(&donor).unwrap().total_donations_by_account
        } else { 
            0 
        };

        /*let to_transfer: Balance = if donated_so_far == 0 {
            // User's first donation, let's register it, which increases storage
            donation_amount - STORAGE_COST
        }
        else {
            donation_amount
        };*/

        donated_so_far += donation_amount;

        self.total_global_amount_donated += donation_amount;
        match self.donations.get(&donor) {
            Some(mut donations_by_acc) => {
                // Donor exists
                log!("account_donations before: {:?}", self.donations.get(&donor).unwrap().account_donations.to_vec());
                log!("self.donations before: {:?}",  self.donations.to_vec());
                
                donations_by_acc.account_donations.push(&donation_msg);
                donations_by_acc.total_donations_by_account = donated_so_far;
                self.donations.insert(&donor, &donations_by_acc);  
                
                log!("account_donations after: {:?}", self.donations.get(&donor).unwrap().account_donations.to_vec());
                log!("self.donations after: {:?}",  self.donations.to_vec());
                
            },
            None => {
                // Donor doesn't exist
                log!("1.account_donations before: {}", "null");
                log!("1.self.donations before: {:?}",  self.donations.to_vec());

                let mut acc_donations: Vector<DonationMessage> = Vector::new(b"c"); 
                acc_donations.push(&donation_msg);

                let donations_by_account: DonationsByAccount = DonationsByAccount {
                    account_donations: acc_donations,
                    total_donations_by_account: donation_amount,
                };
                self.donations.insert(&donor, &donations_by_account);

                log!("1.account_donations after: {:?}", self.donations.get(&donor).unwrap().account_donations.to_vec());
                log!("1.self.donations after: {:?}",  self.donations.to_vec());
            }
        }
        
        log!("Thank you {} for donating {}! You donated a total of {}", donor.clone(), donation_amount, donated_so_far);
        log!("total_global_amount_donated: {}", self.total_global_amount_donated);
        // Send the money to the beneficiary
        // Promise::new(beneficiary).transfer(to_transfer);

        donation_msg
    }

    #[private]
    //#[payable] // Public - People can attach money
    pub fn withdraw(&mut self, beneficiary: AccountId, amount_to_withdraw: U128) -> U128 {
        let total_storage_cost_used: Balance = self.donations.len() as u128 * STORAGE_COST;
        let real_global_transferable_amount: Balance = if self.total_global_amount_donated > total_storage_cost_used {
            self.total_global_amount_donated - total_storage_cost_used
        } else {
            0
        };
        
        assert!(u128::from(amount_to_withdraw) <= real_global_transferable_amount, "Withdraw aborted. Amount to withdraw {} cannot exceed {}", u128::from(amount_to_withdraw), real_global_transferable_amount);
        
        Promise::new(beneficiary.clone()).transfer(u128::from(amount_to_withdraw));
        self.total_global_amount_donated -= u128::from(amount_to_withdraw);
        
        return amount_to_withdraw;
    }
    
    pub fn join_the_lottery(&mut self, account_to_register: AccountId) {
        self.registered_lottery_accounts.insert(&account_to_register);        
    }
    
    pub fn is_account_registered_for_lottery(&self, account_to_check: AccountId) -> bool {
        return self.registered_lottery_accounts.contains(&account_to_check);
    }
    
    // Generate random number from 1 to limit (limit not included)
    pub fn get_random_number(&self, limit: u128) -> u128 {
        const FIRST_N: usize = 10;

        let random_seed: Vec<u8> = env::random_seed(); // len 32 (each value can be from 0 to 255)
        log!("random_seed len: {}", random_seed.len());
        log!("random_seed: {:?}", random_seed);
        // slice containing FIRST_N numbers from 0 to 9
        let random_slice: Vec<u8> = random_seed[0..FIRST_N].iter().map(|x| x % 10).collect();
        let random_string: String = random_slice.into_iter().map(|x| x.to_string()).collect::<String>();
        
        // comvert to u128 to prepare for final number
        let random_number: u128 = random_string.parse::<u128>().expect("Random number invalid!");
        let final_number: u128 = random_number % limit;
        
        return final_number;
    }

    pub fn pick_lottery_winner_and_withdraw(&mut self) -> AccountId {
        // Choose a random beneficiary among lottery registered users
        let registered_users_count: u64 = self.registered_lottery_accounts.len();
        log!("registered_users_count: {}", registered_users_count);
        require!(registered_users_count > 0, "No account registered for the lottery!");
        let index_picked: usize  = self.get_random_number(registered_users_count as u128) as usize;
        log!("Picked index: {}", index_picked);

        let vec_registered_accounts: Vec<AccountId> = self.registered_lottery_accounts.to_vec();
        let picked_beneficiary: AccountId = vec_registered_accounts[index_picked].clone();
        log!("picked_beneficiary: {}", picked_beneficiary.to_string());
        let amount_to_withdraw: u128 = (self.total_global_amount_donated as f32 * 0.99) as u128;
        log!("amount_to_withdraw: {}", amount_to_withdraw);
        self.withdraw(picked_beneficiary.clone(), U128(amount_to_withdraw));
        log!("Congratulations {}! You won the lottery this month!", picked_beneficiary.to_string());
        self.registered_lottery_accounts.clear();

        return picked_beneficiary;
    }
}
