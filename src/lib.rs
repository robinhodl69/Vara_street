
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
#[derive(Debug, Clone, Default)]
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
        // add function 
    }

    #[allow(dead_code)]
    pub fn withdraw_funds(&mut self, amount: u128, lender: UserLender) {
        // add function
    }

    #[allow(dead_code)]
    pub fn borrow(&mut self, amount: u128, borrower: UserBorrower) {
        // Add function
    }

    #[allow(dead_code)]
    pub fn repay(&mut self, amount: u128, borrower: UserBorrower) {
        // add function
    }
    
    #[allow(dead_code)]
    pub fn liquidate(&mut self, loan: Loans, liquidator: UserLender) {
            // Add function
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