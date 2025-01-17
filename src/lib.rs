
#![no_std]
use gstd::{exec, msg , prelude::*,ActorId, async_main};
use io::*;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));


// 1. The main state as a static variable.
static mut STATE: Option<GlobalState> = None;

static mut ADDRESSFT:Option<InitFT> = None;


// 2. The mutability function for state.
fn state_mut() -> &'static mut GlobalState {

    let state = unsafe {  STATE.as_mut()};

    unsafe { state.unwrap_unchecked() }


}


fn ft_state_mut() -> &'static mut InitFT {

    let addressft = unsafe { ADDRESSFT.as_mut()};

    unsafe { addressft.unwrap_unchecked() }


}

// 3. Public State
#[derive(Clone, Default)]
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

    //transfer collateral to user - withdraw
    async fn synthetic_transfer_from_to_user(&mut self, amount: u128) {
 
        let _source = msg::source();
        let _current_globalstate =state_mut();
        let syntheticasset_programid = ft_state_mut();           
        let payload = FTAction::Transfer{from: exec::program_id(), to: msg::source() ,amount: amount};
        let _ = msg::send(syntheticasset_programid.syntheticasset_programid, payload, 0);
       

    }

    async fn  synthetic_transfer_to_contract(&mut self, amount: u128) {
 
        let _source = msg::source();
        let _current_globalstate =state_mut();
        let syntheticasset_programid = ft_state_mut();       
        let payload = FTAction::Transfer{from: msg::source(), to: exec::program_id(), amount: amount};
        let _ = msg::send(syntheticasset_programid.syntheticasset_programid, payload, 0);
       
    }


    async fn stablecoin_transfer_from_to_user(&mut self, amount: u128) {
 
        let  source = msg::source();
        let _current_globalstate =state_mut();
        let stablecoin_programid = ft_state_mut();          
        let payload = FTAction::Transfer{from: exec::program_id(), to: source  ,amount: amount};
        let _ = msg::send(stablecoin_programid.stablecoin_programid, payload, 0);
       

    }
  
    async fn stablecoin_transfer_to_contract(&mut self, amount: u128) {
 
        let source = msg::source();
        let _current_globalstate =state_mut();
        let stablecoin_programid = ft_state_mut();           
        let payload = FTAction::Transfer{from: source, to: exec::program_id(), amount: amount};
        let _ = msg::send(stablecoin_programid.stablecoin_programid, payload, 0);
       
    }

    
   




    #[allow(dead_code)]
    async fn deposit_synthetic(&mut self, amount: u128) {
    
        // Create a variable with mutable state.
        let current_globalstate = state_mut();

        let collateral_available = (amount*50)/100;
                
        // Update state
        current_globalstate.borrowers
        .entry(msg::source())
        .and_modify(|borrower| {
            // If the lender exists, update the balance
            borrower.loanamount   = borrower.loanamount.saturating_add(collateral_available);
           
        })
        .or_insert(
            UserBorrower {
                status: LoanStatus::Active, 
                loanamount:collateral_available,   
                ltvratio: 50, 
                ..Default::default()
        });

     
        // Increase the total deposited amount
        current_globalstate.total_syntetic_deposited = current_globalstate.total_syntetic_deposited.saturating_add(amount);


    }



    #[allow(dead_code)]
    async fn deposit_collateral(&mut self, amount: u128) {
    
        // Create a variable with mutable state.
        let current_globalstate = state_mut();
                
        // Update state
        current_globalstate.lenders
        .entry(msg::source())
        .and_modify(|lender| {
            // If the lender exists, update the balance
            lender.liquidity   = lender.liquidity.saturating_add(amount);
           
        })
        .or_insert(
            UserLender {
                status: UserStatus::Active,
                liquidity: amount,
                ..Default::default()
        });

        // Increase the total deposited amount
        current_globalstate.total_stablecoin_deposited = current_globalstate.total_stablecoin_deposited.saturating_add(amount);


    }

    #[allow(dead_code)]
    async fn withdraw_collateral(&mut self, amount: u128) {
    
        // Create a variable with mutable state.
        let current_globalstate = state_mut();
                
        // Update state
        current_globalstate.lenders
        .entry(msg::source())
        .and_modify(|lender| {
            // If the lender exists, update the balance
            lender.liquidity   = lender.liquidity.saturating_sub(amount);
           
        });

        // Increase the total deposited amount
        current_globalstate.total_stablecoin_deposited = current_globalstate.total_stablecoin_deposited.saturating_sub(amount);


    }

    #[allow(dead_code)]
    pub fn withdraw_funds(&mut self, _amount: u128) {
     
    }

    #[allow(dead_code)]
    pub fn borrow(&mut self, _amount: u128) {
      
    }

    #[allow(dead_code)]
    pub fn repay(&mut self, _amount: u128) {
          }
    
    #[allow(dead_code)]
    pub fn liquidate(&mut self) {
            // Add function - Pending
        }
    }

// 5. Create the init() function of your contract.
#[no_mangle]
extern "C" fn init() {

     let config: InitFT = msg::load().expect("Unable to decode InitFT");

     let state = GlobalState {
        ..Default::default()
    };
 
  
     if config.syntheticasset_programid.is_zero() {
         panic!("FT program address can't be 0");
     }


     if config.stablecoin_programid.is_zero() {
        panic!("FT program address can't be 0");
    }

    
     let initft = InitFT {
        syntheticasset_programid: config.syntheticasset_programid,
        stablecoin_programid: config.stablecoin_programid

     };
    
     
     unsafe {
         ADDRESSFT = Some(initft);
     }
 
    unsafe { STATE = Some(state) };
}

// 4.Create the Handle() function of your contract. Aqui
#[async_main]
async fn main(){

        // We load the input message
        let action = msg::load().expect("Could not load Action");

        let current_globalstate = unsafe {STATE.get_or_insert(GlobalState::default()) };
      
        // We receive an action from the user and update the state. Example:
        match &action {
            Action::DepositFunds(amount) => {


                current_globalstate.deposit_collateral(*amount).await;

                current_globalstate.stablecoin_transfer_to_contract(*amount).await;


                },

            #[allow(dead_code)]
            Action::WithdrawFunds(amount) => {

                current_globalstate.withdraw_collateral(*amount).await;

                current_globalstate.stablecoin_transfer_from_to_user(*amount).await;

               
               

            },

            #[allow(dead_code)]
            Action::Borrow(amount) => {

                current_globalstate.deposit_synthetic(*amount).await;
                
                current_globalstate.synthetic_transfer_to_contract(*amount).await;
               
            
            },

            #[allow(dead_code)]
            Action::Repay(_amount) => {
               
         
            },

            #[allow(dead_code)]
            Action::Liquidate(_amount) => {
               
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
    let liquidity_status = liquidity_status.iter().map(|(k, v)| (*k, v.clone())).collect();
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