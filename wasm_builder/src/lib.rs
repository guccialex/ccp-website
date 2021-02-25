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
    
    dragged: Option<Dragged>,
    
    //the pieces and values of each put up to offer
    //for either a need to raise, check, or settle the debt
    piecesforoffer: HashSet<u16>,
}

impl ClientState{
    
    
    fn new() -> ClientState{
        
        ClientState{
            
            selectedobject: None,
            dragged: None,    
            piecesforoffer: HashSet::new(),
        }
        
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
    
    pub fn new(playerid: u8) -> FullGame{
        
        //set the panic hook so i get real error reporting
        panic::set_hook( Box::new(console_error_panic_hook::hook) );


        
        FullGame{
            
            localgame: LocalGameInterface::new(playerid),
            
            queuedoutgoingsocketmessages: Vec::new(),
            
            clientstate: ClientState::new(),
        }
        
    }
    
    //give this wasm struct a message from the server
    pub fn get_incoming_socket_message(&mut self, message: String){
        
        //let backtovecofchar = message.chars().collect::<Vec<_>>();
        //let backtogamebin = backtovecofchar.iter().map(|c| *c as u8).collect::<Vec<_>>();
        
        self.localgame.receive_game_update( message );
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
        
        //tick
        self.localgame.tick();
    }
    
    //return an object with the data of what the game should look like currently
    pub fn get_appearance_data(&mut self) -> JsValue{
        
        //use the interface and the the client state to get the appearance of the game
        //and return it as a javascript object to the client
        let mut toreturn = self.localgame.get_full_appearance_state( &self.clientstate );
        
        
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
                    
                    if self.localgame.can_object_be_dragged(selectedobject){
                        return true;
                    }
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
        
        
        //if an object is selected
        if let Some(selectedobject) = self.clientstate.selectedobject{
            
            //if the selected object can be dragged
            if self.localgame.can_object_be_dragged(selectedobject){
                
                
                let objectover = ObjectType::from_objectname(objectovername);

                self.clientstate.dragged = Some( Dragged{
                   relativepos: (relativedistancex, relativedistancey),
                   objectover: objectover,
                });
            }            
        }
    }
    
    
    //the mouse is raised
    pub fn mouse_up(&mut self){
        
        //if the selected object is being dragged
        if let Some( dragged ) = &self.clientstate.dragged {

            let (relativex, relativey) = dragged.relativepos;
            
            //if there is a selected object and it is a piece
            if let Some( ObjectType::piece(pieceid) ) = self.clientstate.selectedobject{
                
                //if the distance its dragged is enough to flick, flick the object and set selected object to none
                if let Some( (forcex, forcey) ) = get_flick_force(relativex, relativey){

                    //try to flick that piece
                    if let Some(input) = self.localgame.try_to_flick_piece(pieceid, forcex, forcey){

                        self.queuedoutgoingsocketmessages.push(input);

                        //unselect the selected object if it is set to be flicked
                        self.clientstate.selectedobject = None;
                    };
                };
            };
        };
        
        
        //clear the object being dragged
        self.clientstate.dragged = None;
    }
    
    
}








//return the distance from the piece
//and the rotation relative to the piece
//if its been dragged far enough to flick
fn get_flick_force(relativedistancex: f32, relativedistancey: f32) -> Option<(f32, f32)>{
    
    //the distance plus the length of half the cue
    let curtotaldistance = (relativedistancex * relativedistancex + relativedistancey * relativedistancey).sqrt();
    
    //if the distance of the que is farther or closer than it should be, change the scalar to render it within range
    let mut distancescalar = 1.0;
    
    //if the distance of the que is less than 2 units away from the piece, make it two units away
    if curtotaldistance <= 1.0{
        distancescalar = 1.0 / curtotaldistance ;
    }
    
    
    let xrotation = relativedistancex.atan2(relativedistancey);
    
    
    if curtotaldistance >= 1.0{
        
        return Some( (-xrotation - (3.14159 / 2.0), (curtotaldistance - 1.0) * 1.0) );
        
    };
    
    
    return None;
    
    
    
}















#[derive(PartialEq, Copy, Clone, Hash, Eq, Debug)]
pub enum ObjectType{
    
    boardsquare(u16),
    piece(u16),
    
    deck,
    
}


impl ObjectType{


    //turn self into the name of the object
    pub fn to_objectname(&self) -> String {
        
        if let ObjectType::piece(pieceid) = self{
            
            let toreturn = "P".to_string() + &pieceid.to_string();
            return toreturn ;
        }
        else if let ObjectType::boardsquare(boardsquareid) = self{
            
            let toreturn = "B".to_string() + &boardsquareid.to_string();
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
        //if the first character of the objects name is "P"
        else if objectname.chars().nth(0).unwrap() == 'P'{
            
            //get the rest of the name and try to convert it to an int
            let stringpieceid = objectname[1..].to_string();
            let intpieceid = stringpieceid.parse::<u16>().unwrap();
            let toreturn = ObjectType::piece(intpieceid);
            
            return Some (toreturn);
            
        }
        //if the first character of the objects name is "B"
        else if objectname.chars().nth(0).unwrap() == 'B'{
            
            //get the rest of the name and try to convert it to an int
            let bsid = objectname[1..].to_string();
            let intbsid = bsid.parse::<u16>().unwrap();
            let toreturn = ObjectType::boardsquare(intbsid);
            
            return Some (toreturn);
        }
        else{
            
            return None;
        }

    }


}






