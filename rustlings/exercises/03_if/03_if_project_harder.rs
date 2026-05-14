fn process_transaction(balance:i64 , amount : i64 , transaction_type: &str) -> i64{
    // we want to make it so that if the transaction type is a depost then we add the amount and return the new balance
    // if the transaction type is a withdraw then we check if amount > balance to see if there is enough money
    // if not return insuffifcient funds and return the current balance
    // if there is money within there then we want to subtract then return the new balance
    // any other parem within the transciton type we return invalid transction and return the same balance


    let transaction_type = transaction_type.to_lowercase();

    if transaction_type== "deposit"{
        balance + amount
    } else if transaction_type == "withdraw"{
        if amount > balance{
            println!("No money");
            balance
        } else{
            balance - amount
        }
    }else{
        println!("Wrong transaction type try again");
        balance
    }


}





fn main(){

    let balance = 90;
    let amount = 100;
    let transaction_type = "wIthDraw";
    let process_transaction = process_transaction(balance,amount,transaction_type);
    println!("balance : {}, amount : {}, transaction_typle : {}",balance,amount,transaction_type);
    println!("Processed Transaction : {}",process_transaction);
}