
#![no_std]
use gstd::{prelude::*, ActorId };
use gmeta::{In, InOut,Metadata};
use hashbrown::HashMap;


#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitFT {
    pub syntheticasset_programid: ActorId,
    pub stablecoin_programid: ActorId,
}


// 1. Actions
#[derive(Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Action {
    //  Actions
    DepositFunds(u128), // User deposit funds into the protocol 
    WithdrawFunds(u128), // User withdraw funds from the protocol 
    Borrow(u128), // User borrows funds from the protocol
    Repay(u128), // User repays a loan
    Liquidate(u128), // A loan is liquidated because the loan to value ratio is lower than the minimum required
}
// 2.  Events
#[derive(Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Event {
    //  Events
    FundsDeposited, // Funds have been deposited into the protocol
    FundsWithdrawn, // Funds have been withdrawn from the protocol
    LoanBorrowed, // A loan has been borrowed
    LoanRepaid, // A loan has been repaid
    LoanLiquidated, // A loan has been liquidated because the loan to value ratio is lower than the minimum required
}


// 3. Borrower struc
#[derive(Debug, Clone)]
pub struct UserBorrower {

   
    status: LoanStatus, // The status of the loan
    loanamount: (u128), // The amount of the loan
    ltvratio: u64, // The loan to Value ratio
    historial: Vec<(u128,Loans)> // The historial of the loans   

}

// 3. Provider struc
#[derive(Debug, Clone)]
pub struct UserLender {
    status: UserStatus, // The status of the lender
    liquidity: u128, // amount of liquidity provided
    loans_given: Vec<(u128, LiquidityStatus)>, // The history of loans given
}

// 3. Loan struc
#[derive(Debug, Clone)]

pub struct Loans  {

    id: u128,    
    amount: u128, // The amount of the loan
    collateral_amount: u128, // The amount of the collateral
    ltv_ratio: u64, // The loan to Value ratio
    closing: LoanStatus, // The status of the loan 
    
    // delayed message como oraculo para MVP - roadmap Oraculo

}



#[derive(Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum LoanStatus {
    #[default]
    Active, // A loan is active
    Inactive, // The loan has been repaid

}



#[derive(Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum LiquidityStatus {
    #[default]
    Active, // A liqudity positive is active
    Inactive, // a liqudity positive is inactive

}


#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum UserStatus {
    #[default]
    Active, // A loan is active
    Inactive, // The loan has been repaid

}



pub struct ContractMetadata;

// 4. Define the structure of actions, events and state for your metadata.
impl Metadata for ContractMetadata {
    type Init = In<InitFT>;
    type Handle = InOut<Action, Event>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = IoGlobalState;
}

// 5. Define the global state
#[derive(Debug, Clone, Default)]
pub struct IoGlobalState  {
    pub borrowers: Vec<(ActorId,UserBorrower)>,
    pub lenders: Vec<(ActorId,UserLender)>,
    pub loans: Vec<(ActorId,Loans)>,
    pub loan_status: Vec<(ActorId,LoanStatus)>,
    pub liquidity_status: Vec<(ActorId,LiquidityStatus)>,
    pub user_status: Vec<(ActorId,UserStatus)>,
}

#[derive(Default, Encode, Decode, Clone, TypeInfo)]
pub struct GlobalState {

    pub borrowers: HashMap<ActorId, UserBorrower>,
    pub lenders: HashMap<ActorId, UserLender>,
    pub loans: HashMap<ActorId, Loans>,
    pub loan_status: HashMap<ActorId, LoanStatus>,
    pub liquidity_status: HashMap<ActorId, LiquidityStatus>,
    pub user_status: HashMap<ActorId, UserStatus>,
    
}