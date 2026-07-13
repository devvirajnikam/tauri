pub struct Account {
    pub owner: String,
    balance: i32,
}

impl Account {
    pub fn new(owner: String, balance: i32) -> Account {
        Account { owner, balance }
    }

    pub fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    pub fn balance(&self) -> i32 {
        self.balance
    }
}

pub fn run() {
    println!("\n14. Visibility with pub");

    let mut account = Account::new(String::from("Asha"), 100);
    account.deposit(50);

    println!("Owner is public: {}", account.owner);
    println!(
        "Balance is private, read through method: {}",
        account.balance()
    );
}
