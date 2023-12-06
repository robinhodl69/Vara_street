
#![no_std]
use gstd::{ msg , prelude::*,ActorId};
use io::*;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));


// 1. The main state as a static variable.
static mut STATE: Option<GlobalState> = None;


// 2. The mutability function for state.
fn state_mut() -> &'static mut GlobalState {

    let state = unsafe {  STATE.as_mut()};

    unsafe { state.unwrap_unchecked() }


}

// 3. Public State
#[derive(Debug, Clone, Default, TypeInfo)]
pub struct GlobalState {

    pub borrowers: HashMap<ActorId, UserBorrower>,
    pub lenders: HashMap<ActorId, UserLender>,
    pub loans: HashMap<ActorId, Loans>,
    pub loan_status: HashMap<ActorId, LoanStatus>,
    pub liquidity_status: HashMap<ActorId, LiquidityStatus>,
    pub user_status: HashMap<ActorId, UserStatus>,
    
}


// 4. Create a implementation on State 
impl GlobalState {
    #[allow(dead_code)]
    pub fn deposit_funds(&mut self, amount: u128, lender: UserLender) {
        let lender = self.lenders.iter_mut().find(|l| *l == lender);
        if let Some(lender) = lender {
            lender.liquidity += amount;
        }
        // Emit FundsDeposited event
    }

    #[allow(dead_code)]
    pub fn withdraw_funds(&mut self, amount: u128, lender: UserLender) {
        let lender = self.lenders.iter_mut().find(|l| *l == lender);
        if let Some(lender) = lender {
            lender.liquidity -= amount;
        }
        // Emit FundsWithdrawn event
    }

    #[allow(dead_code)]
    pub fn borrow(&mut self, amount: u128, borrower: UserBorrower) {
        let loan = Loans { id: self.loans.len() as u128, amount, closing: LoanStatus::Active };
        let borrower = self.borrowers.iter_mut().find(|b| *b == borrower);
        if let Some(borrower) = borrower {
            borrower.loanamount += amount;
            borrower.historial.push((loan.id, loan));
        }
        // Emit LoanBorrowed event
    }

    #[allow(dead_code)]
    pub fn repay(&mut self, amount: u128, borrower: UserBorrower) {
        let borrower = self.borrowers.iter_mut().find(|b| *b == borrower);
        if let Some(borrower) = borrower {
            borrower.loanamount -= amount;
            // Find the loan and update its amount or status
        }
        // Emit LoanRepaid event
    }

    #[allow(dead_code)]
    pub fn set_liquidation_threshold(&mut self, new_threshold: u128) {
        // TODO: Check that the caller is the owner of the contract
        self.liquidation_threshold = new_threshold;
    }
    
    #[allow(dead_code)]
    pub fn liquidate(&mut self, loan: Loans, liquidator: UserLender) {
        // TODO: Calculate the loan to value
        let loan_to_value = /* ... */;
    
        if loan_to_value <= self.liquidation_threshold {
            let loan = self.loans.iter_mut().find(|l| *l == loan);
            if let Some(loan) = loan {
                loan.closing = LoanStatus::Inactive;
            }
            // Emit LoanLiquidated event
        }
    }
}

// 5. Create the init() function of your contract.
#[no_mangle]
extern "C" fn init() {
    let state = GlobalState {
        ..Default::default()
    };

    unsafe { STATE = Some(state) };
}

// 4.Create the Handle() function of your contract. Aqui
#[no_mangle]
extern "C" fn handle(){

        // We load the input message
        let action: Action = msg::load().expect("Could not load Action");

        // We receive an action from the user and update the state. Example:
        match &action {
            Action::FirstAction => {

                // Create a variable with mutable state.
                let currentstate = state_mut();

                // Update your state.
                currentstate.firstfield = "Update".to_string();


                 // Generate your event.
                let _ = msg::reply(Event::FirstEvent,0);


            }
            Action::SecondAction => {

                let currentstate = state_mut();

                currentstate.firstfield = "Update".to_string();

                let _ = msg::reply(Event::SecondEvent,0);
               

            }
            Action::ThirdAction => {
               
                let currentstate = state_mut();

                currentstate.firstfield = "Update".to_string();

                let _ =  msg::reply(Event::ThirdEvent,0);
            }
        };
    }

        


// 5. Create the state() function of your contract.
#[no_mangle]
extern "C" fn state() {
    let state = unsafe { STATE.take().expect("Unexpected error in taking state") };
    msg::reply::<IoGlobalState>(state.into(), 0)
    .expect("Failed to encode or reply with `<ContractMetadata as Metadata>::State` from `state()`");
}

impl From<GlobalState> for IoGlobalState {
    fn from(value: GlobalState) -> Self {

    let GlobalState {

        borrowers,
        lenders,
        loans,
        loan_status,
        liquidity_status,
        user_status, 
       
    } = value;

    let borrowers = borrowers.iter().map(|(k, v)| (*k, v.clone())).collect();
    let lenders = lenders.iter().map(|(k, v)| (*k, v.clone())).collect();
    let loans = loans.iter().map(|(k, v)| (*k, v.clone())).collect();
    let loan_status = loan_status.iter().map(|(k, v)| (*k, v.clone())).collect();
    let liqidity_status = loan_status.iter().map(|(k, v)| (*k, v.clone())).collect();
    let user_status = user_status.iter().map(|(k, v)| (*k, v.clone())).collect();

    Self {

        borrowers,
        lenders,
        loans,
        loan_status,
        liquidity_status,
        user_status, 
        
    }

}
}