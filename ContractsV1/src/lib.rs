
#![no_std]
use gstd::{exec, msg , prelude::*,ActorId, async_main};
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

    pub total_syntetic_deposited:u128,
    pub total_stablecoin_deposited:u128,
    pub borrowers: HashMap<ActorId, UserBorrower>,
    pub lenders: HashMap<ActorId, UserLender>,
    pub loans: HashMap<ActorId, Loans>,
    pub loan_status: HashMap<ActorId, LoanStatus>,
    pub liquidity_status: HashMap<ActorId, LiquidityStatus>,
    pub user_status: HashMap<ActorId, UserStatus>,
    
}


// 4. Create a implementation on State 
impl GlobalState {

    //transfer collateral to user - withdraw funds
    async fn tokens_transfer_from_to_user(&mut self, amount: u128) {
 
        let _source = msg::source();
        let _current_globalstate =state_mut();
        let synthetic_programid = synthetic_state_mut();           
        let payload = FTAction::Transfer{from: exec::program_id(), to: msg::source() ,amount: amount};
        let _ = msg::send(address_ft.ft_program_id, payload, 0);
       

    }
    //transfer collateral to contract - deposit
    async fn tokens_transfer_to_contract(&mut self, amount: u128) {
 
        let _source = msg::source();
        let _current_globalstate =state_mut();
        let synthetic_programid = synthetic_state_mut();           
        let payload = FTAction::Transfer{from: msg::source(), to: exec::program_id(), amount: amount};
        let _ = msg::send(synthetic_programid.ft_program_id, payload, 0);
       

    }


    #[allow(dead_code)]
    pub fn deposit_collateral(&mut self, amount: u128) {
    
        // Create a variable with mutable state.
        let current_globalstate = state_mut();
                
        // Update state
        current_globalstate.lenders
        .entry(msg::source())
        .and_modify(|lender| {
            // If the lender exists, update the balance
            current_globalstate.lenders.liquidity = current_globalstate.lenders.balance.saturating_add(amount);
            current_globalstate.lenders = lender.loans_given.push(amount,LiquidityStatus::Active);
        })
        .or_insert(
            UserLender {
                status: UserStatus::Active,
                liquidity: amount,
               ..Default
        });

        // Transfer to Contract

        current_globalstate.tokens_transfer_to_contract(amount);

        // Increase the total deposited amount
        current_globalstate.total_syntetic_deposited = current_globalstate.total_syntetic_deposited.saturating_add(amount);


    }

    #[allow(dead_code)]
    pub fn withdraw_funds(&mut self, amount: u128, lender: UserLender) {
     
    }

    #[allow(dead_code)]
    pub fn borrow(&mut self, amount: u128, borrower: UserBorrower) {
      
    }

    #[allow(dead_code)]
    pub fn repay(&mut self, amount: u128, borrower: UserBorrower) {
          }
    
    #[allow(dead_code)]
    pub fn liquidate(&mut self, loan: Loans, liquidator: UserLender) {
            // Add function - Pending
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
#[async_main]
async fn main(){

        // We load the input message
        let action = msg::load().expect("Could not load Action");
      


        // We receive an action from the user and update the state. Example:
        match &action {
            Action::DepositFunds(amount) => {

                current_globalstate.deposit_funds(amount);


                },

            #[allow(dead_code)]
            Action::WithdrawFunds(amount) => {

               
               

            },

            #[allow(dead_code)]
            Action::Borrow(amount) => {
               
            
            },

            #[allow(dead_code)]
            Action::Repay(amount) => {
               
         
            },

            #[allow(dead_code)]
            Action::Liquidate(amount) => {
               
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

        total_syntetic_deposited,
        total_stablecoin_deposited,
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
    let liqidity_status = liquidity_status.iter().map(|(k, v)| (*k, v.clone())).collect();
    let user_status = user_status.iter().map(|(k, v)| (*k, v.clone())).collect();

    Self {
        total_syntetic_deposited,
        total_stablecoin_deposited,
        borrowers,
        lenders,
        loans,
        loan_status,
        liquidity_status,
        user_status, 
        
    }

}
}