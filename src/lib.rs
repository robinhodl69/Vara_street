
#![no_std]
use gstd::{ msg , prelude::*,ActorId};
use io::*;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));


// 1. Create the main state as a static variable.
static mut STATE:Option<CustomStruct> = None;



// 2. Create the mutability function for your state.
fn state_mut() -> &'static mut CustomStruct {

    let state = unsafe {  STATE.as_mut()};

    unsafe { state.unwrap_unchecked() }


}

// Create a public State
#[derive(Default, Encode, Decode, TypeInfo)]
pub struct CustomStruct {
    pub firstfield: String,
    pub secondfield: u128,
    pub thirdfield: ActorId,
}

// Create a implementation on State
impl CustomStruct {
    #[allow(dead_code)]
    fn firstmethod(&mut self) {}
    #[allow(dead_code)]
    fn secondmethod(&mut self) { }
    #[allow(dead_code)]
    fn thirdmethod(&mut self) {}
}


// 3. Create the init() function of your contract.
#[no_mangle]
extern "C" fn init () {

    let state = CustomStruct {
        ..Default::default()
    };

    unsafe { STATE = Some(state) };


}


// 4.Create the Handle() function of your contract.
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
    let state = unsafe {
        STATE.get_or_insert(Default::default())
    };
    msg::reply(state, 0).expect("Failed to share state");
}