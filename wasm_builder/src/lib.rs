//the rust project that is compiled into webassembly to be used by the javascript

use std::panic;

use wasm_bindgen::prelude::*;





#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}



#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    
    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}








use std::collections::HashSet;


//if the selected object is being dragged
//the relative distance its being dragged
//and the object that it is being dragged over
#[derive(Clone)]
struct Dragged{
    
    relativepos: (f32,f32),
    objectover: Option<ObjectType>,

}




pub struct ClientState{
    //the name of the object that is selected
    selectedobject: Option<ObjectType>,

    waitingforopponent: bool,
}

impl ClientState{
    
    
    fn new() -> ClientState{
        
        ClientState{
            
            selectedobject: None,

            waitingforopponent: true,
        }
    }


    fn opponent_connected(&mut self){

        self.waitingforopponent = false;
    } 
    
}


mod gameinterface;

use gameinterface::LocalGameInterface;


#[wasm_bindgen]
pub struct FullGame{
    
    //the local client side version of the game if it exists
    localgame: LocalGameInterface,
    
    queuedoutgoingsocketmessages: Vec<String>,
    
    clientstate: ClientState,

}






#[wasm_bindgen]
impl FullGame{

    
    pub fn new(playerid: u8, offlinegame: bool) -> FullGame{
        
        //set the panic hook so i get real error reporting
        //panic::set_hook( Box::new(console_error_panic_hook::hook) );
        console_error_panic_hook::set_once();

        let mut clientstate;

        if offlinegame == true{

            clientstate = ClientState::new();
            clientstate.opponent_connected();
        }
        else{
            clientstate = ClientState::new();
        }


        
        FullGame{
            
            localgame: LocalGameInterface::new(playerid),
            
            queuedoutgoingsocketmessages: Vec::new(),
            
            clientstate: clientstate,
        }
        
    }



    
    //give this wasm struct a message from the server
    pub fn get_incoming_socket_message(&mut self, message: String){
        
        self.localgame.receive_game_update( message );

        self.clientstate.opponent_connected();
    }
    
    //if there is an outgoing socket message to pop
    pub fn is_outgoing_socket_message_queued(&self) -> bool{
        
        !self.queuedoutgoingsocketmessages.is_empty()
    }
    
    pub fn pop_outgoing_socket_message(&mut self) -> String{
        
        //get and remove the first element
        self.queuedoutgoingsocketmessages.remove(0)
    }
    

    pub fn tick(&mut self){
    
        self.localgame.tick();
    }
    
    
    //return an object with the data of what the game should look like currently
    pub fn get_appearance_data(&mut self) -> JsValue{
        
        //use the interface and the the client state to get the appearance of the game
        //and return it as a javascript object to the client
        let toreturn = self.localgame.get_full_appearance_state( &self.clientstate );
        
        
        //turn it into a json object and return as a jsvalue
        JsValue::from_serde( &toreturn ).unwrap()
    }
    
    
    //return whether the object passed in is selected and can be dragged
    pub fn is_object_selected_and_draggable(&self, objectname: String) -> bool{
        
        //if the picked object is a valid object
        if let Some(pickedobject) = ObjectType::from_objectname(objectname){
            
            //if there is an object selected
            if let Some(selectedobject) = self.clientstate.selectedobject{
                
                //if the picked object and the already selected object are the same
                if selectedobject == pickedobject{
                    
                    //if that object can be dragged
                    return false;
                }
            }
        }
        
        return false;        
    }    
    
    
    //player input functions
    
    
    
    //a player clicks on an object
    pub fn mouse_down(&mut self, objectname: String){
        
        
        
        //if it can be converted from an object name to an objecttype
        if let Some(pickedobject) =  ObjectType::from_objectname(objectname){
            
            
            
            
            //if theres an object already selected
            if let Some(currentlyselectedobject) = self.clientstate.selectedobject{
                
                if let Some(input) = self.localgame.try_to_perform_action(currentlyselectedobject, pickedobject){
                    
                    self.queuedoutgoingsocketmessages.push(input);
                }
                
                self.clientstate.selectedobject = None;
            }
            
            //if its name is "deck", draw
            else if ObjectType::deck == pickedobject{
                
                if let Some(input) = self.localgame.try_to_draw_card(){
                    self.queuedoutgoingsocketmessages.push(input);
                }
            }
            
            //if the selected object is currently none
            else if self.clientstate.selectedobject == None{
                
                //if the picked object is selectable by me
                if self.localgame.is_object_selectable(pickedobject){
                    
                    //set that object to be the selected one
                    self.clientstate.selectedobject = Some( pickedobject );
                }
            }
            
            
            
        }
        //if its not an object
        else{
            self.clientstate.selectedobject = None;
        }
        
        
    }
    
    
    
    
    
    //if the mouse is being dragged
    //and what object its being dragged over
    //and how far its being dragged
    pub fn drag_selected_object(&mut self, relativedistancex: f32, relativedistancey: f32, objectovername: String ){
        
    }
    
    
    //the mouse is raised
    pub fn mouse_up(&mut self){
        
    }
    
}






















#[derive(PartialEq, Copy, Clone, Hash, Eq, Debug)]
pub enum ObjectType{
    
    object(u16),
    deck,
}


impl ObjectType{


    //turn self into the name of the object
    pub fn to_objectname(&self) -> String {
        
        if let ObjectType::object(id) = self{
            
            let toreturn = "K".to_string() + &id.to_string();
            return toreturn ;
        }
        else if let ObjectType::deck = self{
            return "deck".to_string();
        }
        else{
            panic!("cant convert object type to a string");
        }
        
    }


    pub fn from_objectname(objectname: String) -> Option<ObjectType>{

        if objectname == "deck"{
            return Some( ObjectType::deck  );
        }
        //if the first character of the objects name is "K"
        else if objectname.chars().nth(0).unwrap() == 'K'{
            
            //get the rest of the name and try to convert it to an int
            let id = objectname[1..].to_string();
            let intid = id.parse::<u16>().unwrap();
            let toreturn = ObjectType::object(intid);
            
            return Some (toreturn);
        }
        else{
            
            return None;
        }

    }


}




