fn main() {
    println!("Welcome on your Rust bank account !");

    let mut balance = 1000.0;
    println!("Your initial bank account has ${balance} available");

    println!("How much money do you want to add");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error while reading input lines");

    let deposit: f64 = input.trim().parse().expect("Please provide a valid number");

    balance += deposit;
    println!("Your bank account has now ${balance}");

    println!("How much money do you want to withdrawal");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error while reading input lines");

    let withdrawal: f64 = input.trim().parse().expect("Please provide a valid number");

    println!("{withdrawal}");
    // consider if user in negative number don't increment
    if (withdrawal < 0.0) || (balance - withdrawal < 0.0) {
        println!("Impossible you can not withdrawal more than what you have ");
    } else {
        balance -= withdrawal;
        println!("Your bank account has now ${balance}");
    }

    let borrowed_balance = &balance;
    println!("Your sold before fee {borrowed_balance}");

    let withdrawal_fees = 10.0;
    balance -= withdrawal_fees;
    println!("Here is you final sold {balance}");
    println!("Thank you for using your Rust bank account!");
    
}
