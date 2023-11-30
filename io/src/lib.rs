
#![no_std]
use gstd::{ prelude::*, ActorId };
use gmeta::{InOut,Metadata};



// 1. Actions
#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Action {
    //  Actions
    DepositFunds, // User deposit funds into the protocol
    WithdrawFunds, // User withdraw funds from the protocol 
    Borrow, // User borrows funds from the protocol
    Repay, // User repays a loan
    Liquidate, // A loan is liquidated because the loan to value ratio is lower than the minimum required
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


// 3. Create your own Struct
#[derive(Default, Encode, Decode, Clone, TypeInfo)]
pub struct Loan {
    borrower: ActorId, // The identity of the borrower
    amount: u128, // The amount of the loan
    due_date: u64, // The loan to Value ratio
    status: LoanStatus, // The loan status
}
   
#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum LoanStatus {
    Active, // El préstamo está activo
    Repaid, // El préstamo ha sido devuelto
    Liquidated, // El préstamo ha sido liquidado
}


pub struct ContractMetadata;

// 4. Define the structure of actions, events and state for your metadata.
impl Metadata for ContractMetadata {
    type Init = ();
    type Handle = InOut<Action, Event>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Loan;

}