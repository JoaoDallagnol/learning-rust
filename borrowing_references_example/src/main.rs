// References and Borrowing
// Safety and Performance
// Borrowing and references are powerful concepts

// Understaning References
// References"Enable you to borrow values without taking ownership.
// Immutable Reference
// Mutable Reference
// Create a Reference by add "&"
// -I- Immutable Reference

fn main() {
    let mut _x: i32 = 5;
    let _r: &mut i32 = &mut _x;
    *_r += 1;

    println!("Value of x: {}", _x);

    //---
    let mut account: BankAccount = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };

    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw money
    account.withdraw(45.5);

    // Immutable borrow to check the balance
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdawing {} from account owned by {}!", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}
