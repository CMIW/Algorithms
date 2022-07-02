// Trait all transaction variants should have the execute function
trait Transaction {
    fn execute(&self);
}

// Transaction varianta type A
#[derive(Debug)]
struct TransactionA<'a>{
    transaction_type: &'a str,
}

impl<'a> TransactionA<'a>{
    pub fn new() -> Self{
        TransactionA { transaction_type: "Transaction A" }
    }

    pub fn step_0(&self){
        println!("{} Step 0.", self.transaction_type)
    }

    pub fn step_1(&self){
        println!("{} Step 1.", self.transaction_type)
    }

    pub fn step_2(&self){
        println!("{} Step 2.", self.transaction_type)
    }

    pub fn step_3(&self){
        println!("{} Step 3.", self.transaction_type)
    }
}

// Implement the transaction trait on variant type A
impl<'a> Transaction for TransactionA<'a> {
    fn execute(&self) {
        println!("{} Executed!", self.transaction_type)
    }
}

// Transaction varianta type B
#[derive(Debug)]
struct TransactionB<'a>{
    transaction_type: &'a str,
}

impl<'a> TransactionB<'a>{
    pub fn new() -> Self{
        TransactionB { transaction_type: "Transaction B" }
    }

    pub fn step_a(&self){
        println!("{} Step A.", self.transaction_type)
    }

    pub fn step_b(&self){
        println!("{} Step B.", self.transaction_type)
    }

    pub fn step_c(&self){
        println!("{} Step C.", self.transaction_type)
    }

    pub fn step_d(&self){
        println!("{} Step D.", self.transaction_type)
    }
}

// Implement the transaction trait on variant type B
impl<'a> Transaction for TransactionB<'a> {
    fn execute(&self) {
        println!("{} Executed!", self.transaction_type)
    }
}

// Builder for the transaction variant type A
#[derive(Debug)]
struct TransactionABuilder;

impl TransactionABuilder {
    pub fn new() -> Self{
        TransactionABuilder {}
    }

    fn build(&self, transaction: &TransactionA){
        transaction.step_0();
        transaction.step_1();
        transaction.step_2();
        transaction.step_3();
        transaction.execute();
    }
}

// Builder for the transaction variant type B
#[derive(Debug)]
struct TransactionBBuilder;

impl TransactionBBuilder {
    pub fn new() -> Self{
        TransactionBBuilder {}
    }

    fn build(&self, transaction: &TransactionB){
        transaction.step_a();
        transaction.step_b();
        transaction.step_c();
        transaction.step_d();
        transaction.execute();
    }
}

// Transaction enum to hold all transaction variants
enum Transactions<'a>{
    TransactionA(TransactionA<'a>),
    TransactionB(TransactionB<'a>)
}

// Transaction factory method
fn transaction_factory(transaction: Transactions){
    match transaction {
        Transactions::TransactionA(transaction) => {
            let transaction_a_builder = TransactionABuilder::new();
            transaction_a_builder.build(&transaction);
        }
        Transactions::TransactionB(transaction) => {
            let transaction_b_builder = TransactionBBuilder::new();
            transaction_b_builder.build(&transaction);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // init al transaction types
        let transaction_a:Transactions = Transactions::TransactionA(TransactionA::new());
        let transaction_b:Transactions = Transactions::TransactionB(TransactionB::new());

        // run the transaction factory for both variants
        transaction_factory(transaction_a);
        transaction_factory(transaction_b);

    }
}
