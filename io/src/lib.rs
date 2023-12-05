
#![no_std]
use gstd::{ prelude::*, ActorId };
use gmeta::{InOut,Metadata};



// 1. Actions
#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Action {
    //  Actions
    DepositFunds(u128), // User deposit funds into the protocol 
    WithdrawFunds(u128), // User withdraw funds from the protocol 
    Borrow(u128), // User borrows funds from the protocol
    Repay(u128), // User repays a loan
    Liquidate(u128), // A loan is liquidated because the loan to value ratio is lower than the minimum required
}
// 2.  Events
#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Event {
    //  Events
    FundsDeposited, // Funds have been deposited into the protocol
    FundsWithdrawn, // Funds have been withdrawn from the protocol
    LoanBorrowed, // A loan has been borrowed
    LoanRepaid, // A loan has been repaid
    LoanLiquidated, // A loan has been liquidated because the loan to value ratio is lower than the minimum required
}


// 3. Borrower struc
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct UserBorrower {

   
    status: LoanStatus, // The status of the loan
    loanamount: (u128), // The amount of the loan
    ltvratio: u64, // The loan to Value ratio
    historial: Vec<(u128,Loans)> // The historial of the loans   

}

// 3. Provider struc
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct UserLender {
    status: UserStatus, // The status of the lender
    liquidity: u128, // amount of liquidity provided
    loans_given: Vec<(u128, LiquidityStatus)>, // The history of loans given
}

// 3. Loan struc
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Loans  {

    id: u128,    
    amount: u128, // The amount of the loan
    closing: LoanStatus, // The status of the loan 


}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum LoanStatus {
    Active, // A loan is active
    Inactive, // The loan has been repaid

}



#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum LiquidityStatus {
    Active, // A liqudity positive is active
    Inactive, // a liqudity positive is inactive

}


#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum UserStatus {
    Active, // A loan is active
    Inactive, // The loan has been repaid

}


pub struct ContractMetadata;

// 4. Define the structure of actions, events and state for your metadata.
impl Metadata for ContractMetadata {
    type Init = ();
    type Handle = InOut<Action, Event>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = GlobalState;
}

// 5. Define the global state
#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
pub struct GlobalState {
    pub borrowers: Vec<UserBorrower>,
    pub lenders: Vec<UserLender>,
    pub loans: Vec<Loans>,
    pub loan_status: Vec<LoanStatus>,
    pub liquidity_status: Vec<LiquidityStatus>,
    pub user_status: Vec<UserStatus>,
    pub liquidation_threshold: u128, // The liquidation threshold - Changed from Vec<u128> to u128

}
