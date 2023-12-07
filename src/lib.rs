
#![no_std]
use gstd::{ msg , prelude::*,ActorId, async_main};
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

    //transfer collateral to user - withdraw
    async fn tokens_transfer_from_to_user(&mut self, amount: u128) {
 
        let _source = msg::source();
        let _current_globalstate =state_mut();
        let synthetic_programid = synthetic_state_mut();           
        let payload = FTAction::Transfer{from: exec::program_id(), to: msg::source() ,amount: amount_tokens};
        let _ = msg::send(address_ft.ft_program_id, payload, 0);
       

    }
    //transfer collateral to contract - deposit
    async fn tokens_transfer_to_contract(&mut self, amount: u128) {
 
        let _source = msg::source();
        let _current_globalstate =state_mut();
        let synthetic_programid = synthetic_state_mut();           
        let payload = FTAction::Transfer{from: msg::source(), to: exec::program_id(), amount: amount_tokens};
        let _ = msg::send(address_ft.ft_program_id, payload, 0);
       

    }

    


    #[allow(dead_code)]
    pub fn deposit_funds(&mut self, amount: u128, lender: UserLender) {
        // Access to register of lenders 
        self.lenders
        .entry(msg::source())
        .and_modify(|lender_info| {
            // If the lender exists, update the balance
            lender_info.balance = lender_info.balance.saturating_add(amount);
        })
        .or_insert(UserLender {
            // if the lender does not exist, create a new one
            balance: amount,
            ..Default::default()
        });

    // Increase the total deposited amount
    self.total_deposited = self.total_deposited.saturating_add(amount);


    }

    #[allow(dead_code)]
    pub fn withdraw_funds(&mut self, amount: u128, lender: UserLender) {
         // Access the lender record
         if let Some(lender_info) = self.lenders.get_mut(&lender) {
            // If the lender exists and has enough balance, decrease their balance
            if lender_info.balance >= amount {
                lender_info.balance = lender_info.balance.saturating_sub(amount);
                // Decrease the total deposited funds
                self.total_deposited = self.total_deposited.saturating_sub(amount);
            } else {
                // Handle the case where the lender does not have enough balance
                // This could be an error or a different kind of handling depending on your requirements
            }
        } else {
            // Handle the case where the lender does not exist
            // This could be an error or a different kind of handling depending on your requirements
        }
    }

    #[allow(dead_code)]
    pub fn borrow(&mut self, amount: u128, borrower: UserBorrower) {
        if let Some(borrower_info) = self.borrowers.get_mut(&borrower) {
            // If the borrower exists, increase their loan amount
            borrower_info.loan_amount = borrower_info.loan_amount.saturating_add(amount);
        } else {
            // If the borrower does not exist, create a new record with the initial loan amount
            self.borrowers.insert(borrower, UserBorrower {
                loan_amount: amount,
                ..Default::default()
            });
        }

        // Increase the total loaned amount
        self.total_loaned = self.total_loaned.saturating_add(amount);    
    }

    #[allow(dead_code)]
    pub fn repay(&mut self, amount: u128, borrower: UserBorrower) {
        // Access the borrower record
        if let Some(borrower_info) = self.borrowers.get_mut(&borrower) {
            // If the borrower exists and has a loan amount greater than or equal to the repayment amount
            if borrower_info.loan_amount >= amount {
                // Decrease their loan amount
                borrower_info.loan_amount = borrower_info.loan_amount.saturating_sub(amount);
                // Decrease the total loaned amount
                self.total_loaned = self.total_loaned.saturating_sub(amount);
            } else {
                // Handle the case where the borrower does not have a large enough loan to repay this amount
                // This could be an error or a different kind of handling depending on your requirements
            }
        } else {
            // Handle the case where the borrower does not exist
            // This could be an error or a different kind of handling depending on your requirements
        }    }
    
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
        let _globalstate = unsafe { STATE.get_or_insert(GlobalState::default()) };

        // We receive an action from the user and update the state. Example:
        match &action {
            Action::DepositFunds(amount) => {


                // Create a variable with mutable state.
                let current_globalstate = state_mut();
                let userlender = current_globalstate.lenders.entry(msg::source()).or_insert(UserLender {
                    status: UserStatus::Active,
                    liquidity: amount,
                    ..default
                });
                
                current_globalstate.deposit_funds(*amount, user_lender);


                }
    

            }
            Action::WithdrawFunds => {

                let currentstate = state_mut();

                currentstate.firstfield = "Update".to_string();

                let _ = msg::reply(Event::SecondEvent,0);
               

            }
            Action::Borrow => {
               
                let currentstate = state_mut();

                currentstate.firstfield = "Update".to_string();

                let _ =  msg::reply(Event::ThirdEvent,0);
            }

            Action::Repay => {
               
                let currentstate = state_mut();

                currentstate.firstfield = "Update".to_string();

                let _ =  msg::reply(Event::FourthEvent,0);
            }

            Action::Liquidate => {
               
                let currentstate = state_mut();

                currentstate.firstfield = "Update".to_string();

                let _ =  msg::reply(Event::FifthEvent,0);
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