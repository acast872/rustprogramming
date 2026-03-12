mod bank_account;

use bank_account::BankAccount;

fn main() {
    let mut account = BankAccount::new(100.0);

    account.deposit(50.0);
    account.withdraw(25.0);

    println!("Current balance: {}", account.balance());
}