# Transaction varianta type A
class TransactionA:
    def __init__(self):
        self.transaction_type = "Transaction A"

    def step_0(self):
        print(self.transaction_type, " Step 0.")

    def step_1(self):
        print(self.transaction_type, " Step 1.")

    def execute(self):
        print(self.transaction_type, " Executed!")

# Transaction varianta type B
class TransactionB:
    def __init__(self):
        self.transaction_type = "Transaction B"

    def step_a(self):
        print(self.transaction_type, " Step A.")

    def step_b(self):
        print(self.transaction_type, " Step B.")

    def execute(self):
        print(self.transaction_type, " Executed!")

# Builder for the transaction variant type A
class TransactionABuilder:
    def __init__(self):
        pass

    def build(self, transaction_a):
        transaction_a.step_0()
        transaction_a.step_1()
        transaction_a.execute()

# Builder for the transaction variant type B
class TransactionBBuilder:
    def __init__(self):
        pass

    def build(self, transaction_b):
        transaction_b.step_a()
        transaction_b.step_b()
        transaction_b.execute()

# Transaction factory method
def transaction_factory(transaction):
    match transaction:
        case TransactionA():
            transaction_a_builder = TransactionABuilder()
            transaction_a_builder.build(transaction)
        case TransactionB():
            transaction_b_builder = TransactionBBuilder()
            transaction_b_builder.build(transaction)

# test
def it_works():
    transaction_a = TransactionA()
    transaction_b = TransactionB()

    transaction_factory(transaction_a)
    transaction_factory(transaction_b)
    pass

if __name__ == '__main__':
    it_works()
