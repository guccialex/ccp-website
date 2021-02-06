//the rust project that is compiled into webassembly to be used by the javascript

use std::panic;

use wasm_bindgen::prelude::*;


mod appearancestate;



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



/*
mod interface;
use interface::LocalGameInterface;
use interface::ObjectType;
use interface::FullAppearanceState;
use interface::AppearanceData;
use interface::objectname_to_objecttype;
use interface::objecttype_to_objectname;

*/








use std::collections::HashSet;


//if the selected object is being dragged
//the relative distance its being dragged
//and the object that it is being dragged over
#[derive(Clone)]
struct Dragged{
    
    relativepos: (f32,f32),

    objectover: Option<ObjectType>,

}




struct ClientState{
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




#[wasm_bindgen]
pub struct FullGame{
    
    //the local client side version of the game if it exists
    localgame: LocalGameInterface,
    
    queuedoutgoingsocketmessages: Vec<String>,
    
    clientstate: ClientState,
}


//roles of "fullgame"



/*

the functions that javascript needs to interact with the game:


new()


get_incoming_socket_message(&mut self, message: String)
-> receivegameupdates

is_outgoing_socket_message_queued(&self) -> bool

pop_outgoing_socket_message(&mut self) -> String





tick(&mut self)

get_appearance_data(&mut self) -> JsValue

is_object_selected_and_flickable(&self, objectname: String)

mouse_down(&mut self, objectname: String)

drag_selected_object(&mut self, relativedistancex: f32, relativedistancey: f32, objectovername: String )

mouse_up(&mut self)

*/






#[wasm_bindgen]
impl FullGame{
    
    pub fn new(playerid: u8) -> FullGame{
        
        //set the panic hook so i get real error reporting
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        
        FullGame{
            
            localgame: LocalGameInterface::new(playerid),
            
            queuedoutgoingsocketmessages: Vec::new(),
            
            clientstate: ClientState::new(),
        }
        
    }
    
    //give this wasm struct a message from the server
    pub fn get_incoming_socket_message(&mut self, message: String){
        
        let backtovecofchar = message.chars().collect::<Vec<_>>();
        let backtogamebin = backtovecofchar.iter().map(|c| *c as u8).collect::<Vec<_>>();
        
        self.localgame.receive_game_update( backtogamebin );
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
        let mut toreturn = self.localgame.get_full_appearance_state(self.clientstate);
        
        
        //turn it into a json object and return as a jsvalue
        JsValue::from_serde( &toreturn ).unwrap()
    }
    
    
    //return whether the object passed in is selected and can be dragged
    pub fn is_object_selected_and_draggable(&self, objectname: String) -> bool{
        
        //if the picked object is a valid object
        if let Some(pickedobject) = objectname_to_objecttype(objectname){
            
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
    
    
    
    
    
    
    fn click_in_value_gathering_mode(&mut self, objecttype: ObjectType){
        
        
        //if its a piece select / deselect it
        if let ObjectType::piece(pieceid) = objecttype{
            
            //if this piece is already offered, remove it from the pieces offered
            if self.clientstate.piecesforoffer.contains(&pieceid){
                self.clientstate.piecesforoffer.remove(&pieceid);
            }
            //if its not
            else {
                
                //if it can be offered, add it to the pieces that can be offered
                if self.localgame.can_piece_be_offered(pieceid){
                    self.clientstate.piecesforoffer.insert(pieceid);
                }
            }
        } 
        //if its the debt owed button
        else if ObjectType::debtbutton == objecttype{
            
            if let Some(input) = self.localgame.try_to_settle_debt(self.clientstate.piecesforoffer){

                self.queuedoutgoingsocketmessages.push(input);
            
                self.clientstate.piecesforoffer = HashSet::new();
            }

        }        
        //if its the check button
        else if ObjectType::checkbutton == objecttype{
            
            if let Some(input) = self.localgame.try_to_check(self.clientstate.piecesforoffer){

                self.queuedoutgoingsocketmessages.push(input);
            
                self.clientstate.piecesforoffer = HashSet::new();
            }
            
        }
        //if its the fold button
        else if ObjectType::foldbutton == objecttype{
            
            if let Some(input) = self.localgame.try_to_fold(){

                self.queuedoutgoingsocketmessages.push(input);
            
                self.clientstate.piecesforoffer = HashSet::new();

            }
        }
        //if its the raise button
        else if ObjectType::raisebutton == objecttype{
            
            if let Some(input) = self.localgame.try_to_raise(self.clientstate.piecesforoffer){

                self.queuedoutgoingsocketmessages.push(input);
            
                self.clientstate.piecesforoffer = HashSet::new();    
            };

        }
        
        
    }
    
    
    
    //player input functions
    
    
    
    //a player clicks on an object
    pub fn mouse_down(&mut self, objectname: String){
        
        
        
        //if it can be converted from an object name to an objecttype
        if let Some(pickedobject) = objectname_to_objecttype(objectname){
            
            
            
            //if there is a card game going on
            if self.localgame.is_cardgame_ongoing(){
                
                //click the pieces in the value gathering mode
                self.click_in_value_gathering_mode(pickedobject);
            }
            
            //if theres an object already selected
            else if let Some(currentlyselectedobject) = self.clientstate.selectedobject{
                
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
                
                
                let objectover = objectname_to_objecttype(objectovername);

                self.clientstate.dragged = Some( Dragged{
                   relativepos: (relativedistancex, relativedistancey),
                   objectover: objectover,
                });
                


                /*
                //if the selectedobject is a piece
                if let ObjectType::piece(_) = selectedobject{

                    
                    //only make a flick mission if the mouse is further away from the piece than 1
                    if let Some( (rotation, distance) ) = get_flick_force(relativedistancex, relativedistancey){
                        self.clientstate.dragged = Some( Dragged::piece( rotation, distance ) );
                    }
                    
                    //get the position of the selected piece
                    let selectedposition = self.localgame.get_object_flat_plane_position(selectedobject);
                    
                    let (position, rotation) = get_position_and_rotation_of_cue_indicator(selectedposition, relativedistancex, relativedistancey);
                }
                */

            }            
        }

    }
    
    
    //the mouse is raised
    pub fn mouse_up(&mut self){
        
        //if the selected object is being dragged
        if let Some( dragged ) = self.clientstate.dragged {

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



fn get_position_and_rotation_of_cue_indicator(piecepos: (f32,f32), reldistx: f32, reldisty: f32) -> ((f32,f32,f32), (f32,f32,f32)){
    
    //the distance plus the length of half the cue
    let curtotaldistance = (reldistx * reldistx + reldisty * reldisty).sqrt();
    
    //if the distance of the que is farther or closer than it should be, change the scalar to render it within range
    let mut distancescalar = 1.0;
    
    //if the distance of the que is less than 2 units away from the piece, make it two units away
    if curtotaldistance <= 1.0{
        distancescalar = 1.0 / curtotaldistance ;
    }
    
    
    //0 + the ratio of the hypotenuse length to x length * cue length
    let xcuedistance = (reldistx / curtotaldistance ) * 1.0 ;
    //0 + the ratio of the hypotenuse length to y length * cue length
    let ycuedistance = (reldisty / curtotaldistance ) * 1.0 ;
    
    
    //i want it to circle around the selected pieces position
    //facing inwards
    
    let xdistancefromselected = (reldistx * distancescalar) + xcuedistance;
    let zdistancefromselected = (reldisty * distancescalar) + ycuedistance;
    
    let xrotation = reldistx.atan2(reldisty);
    
    
    
    let position = (piecepos.0 + xdistancefromselected, 0.8, piecepos.1 + zdistancefromselected);
    let rotation = (0.0, xrotation, 0.0);
    
    
    
    return (position, rotation) ;
    
}















use physicsengine::PlayerInput;
use physicsengine::Card;

use std::collections::HashMap;

use physicsengine::MainGame;

use physicsengine::PieceAction;
use physicsengine::BlackJackAction;
use physicsengine::PokerAction;
use physicsengine::CardAction;

//use std::collections::HashSet;



//the interface the "fullgame" has with the rust chesscheckers game
pub struct LocalGameInterface{
    
    
    //the id of the player
    playerid: u8,
    
    //the actual rust game
    thegame: MainGame,
    
    
}


//What methods do I want the game to interface with?
impl LocalGameInterface{
    
    
    //create a game with a certain ID
    pub fn new(playerid: u8) -> LocalGameInterface{
        
        
        let thegame = MainGame::new_two_player();
        
        LocalGameInterface{
            
            playerid: playerid,
            thegame:thegame,
            
        }
    }
    
    
    //tick the local game
    pub fn tick(&mut self) {
        
        self.thegame.tick();
        
    }
    
    
    
    
    
    
    
    
    //inputs:
    
    
    //given the id of an main object, and then an object that its trying to perform an action on
    //return if an input was sent to the game, and if it was, what the serialized string of it is
    pub fn try_to_perform_action(&mut self, object1: ObjectType, object2: ObjectType) -> Option<String>{
        
        
        //if object 1 and 2 are the same card, play that card
        if let ObjectType::card(cardid1) = object1{
            
            if let ObjectType::card(cardid2) = object2{
                
                if cardid1 == cardid2{
                    
                    return  self.try_to_play_card(cardid1) ;
                }
            }
        }
        
        
        
        let objecttoinput = self.get_inputs_of_object(object1);
        
        //if there is a player input that lets object1 perform some action on object 2
        if let Some(playerinput) = objecttoinput.get(&object2){
            
            return self.try_to_perform_input(*playerinput);
        };
        
        
        //otherwise do nothing and return false
        return None;
    }
    pub fn try_to_flick_piece(&mut self, pieceid: u16, direction: f32, force: f32 ) -> Option<String>{
        
        let flickaction = PieceAction::flick(direction, force.sqrt() * 3.0);
        let flickinput = PlayerInput::pieceaction(pieceid, flickaction);
        
        //give the flick input to the game
        return self.try_to_perform_input(flickinput);
        
    }
    pub fn try_to_play_card(&mut self, cardid: u16) -> Option<String>{
        
        let action = CardAction::playcardonboard;
        let input = PlayerInput::cardaction(cardid, action);
        
        
        return self.try_to_perform_input(input);
        
    }
    pub fn try_to_draw_card(&mut self) -> Option<String>{
        
        let input = PlayerInput::drawcard;
        
        return self.try_to_perform_input(input);
        
    }
    pub fn try_to_check(&mut self, pieces: HashSet<u16>) -> Option<String>{
        
        let pieces: Vec<u16> = pieces.clone().into_iter().collect();
        
        let action = PokerAction::check(pieces);
        let input = PlayerInput::pokeraction(action);
        
        
        return self.try_to_perform_input(input);
    }
    pub fn try_to_raise(&mut self, pieces: HashSet<u16>) -> Option<String>{
        
        let pieces: Vec<u16> = pieces.clone().into_iter().collect();
        
        let action = PokerAction::raise(pieces);
        let input = PlayerInput::pokeraction(action);
        
        return self.try_to_perform_input(input);
    }
    pub fn try_to_fold(&mut self) -> Option<String>{
        
        let action = PokerAction::fold;
        let input = PlayerInput::pokeraction(action);
        
        return self.try_to_perform_input(input);
    }
    pub fn try_to_settle_debt(&mut self, pieces: HashSet<u16>) -> Option<String>{
        
        let pieces: Vec<u16> = pieces.clone().into_iter().collect();
        
        let input = PlayerInput::settledebt(pieces);
        
        
        return self.try_to_perform_input(input);
    }
    
    
    
    fn try_to_perform_input(&mut self, playerinput: PlayerInput) -> Option<String>{
        
        //give the flick input to the game

        if let Some(validinput) = self.thegame.receive_input(self.playerid, playerinput.clone()){

            return Some(validinput);


        }
        else{
            return None;
        }
        
    }
    
    




    pub fn get_full_appearance_state(&mut self, clientstate: ClientState) -> FullAppearanceState{
        
        
        let mut toreturn = FullAppearanceState::new();
        
        
        //get the piece ids
        //get the board square ids
        //get the card ids of the cards in the players main hands
        let objectids = self.get_objects();
        
        //get the object appearance of these objects
        let mut objectsappearance: Vec<AppearanceData> = Vec::new();
        
        for objectid in objectids{
            let objectappearance = self.get_object_appearance(objectid);
            objectsappearance.push ( objectappearance );
        };
        
        
        //for every object in the game add it to the full appearance to return
        for objectappearance in objectsappearance{
            toreturn.add_object(objectappearance);
        };        
        
        
        
        
        let deckappearance = AppearanceData::new_deck();
        toreturn.add_object(deckappearance);
        
        //add the appearance of the timer for the player and the opponent
        let player1totaltimeleft = self.thegame.get_players_total_ticks_left(1);
        let iscurrentlyturn = (self.thegame.get_players_turn_ticks_left(1) > 0);
        let player1timer = AppearanceData::new_timer(1, player1totaltimeleft, iscurrentlyturn);
        toreturn.add_object(player1timer);
        
        
        let player2totaltimeleft = self.thegame.get_players_total_ticks_left(2);
        let iscurrentlyturn = (self.thegame.get_players_turn_ticks_left(2) > 0);
        let player2timer = AppearanceData::new_timer(2, player2totaltimeleft, iscurrentlyturn);
        toreturn.add_object(player2timer);
        
        
        
        let debtowed = self.thegame.get_debt_of_player(&self.playerid);
        
        if debtowed != 0{
            toreturn.add_object( AppearanceData::new_debt_owed_button(debtowed) );
            
        }
        else{
            
            
            //if theres a poker game going on
            //give the check, fold and raise buttons
            if self.thegame.is_pokergame_ongoing() {
                
                let checkbutton = AppearanceData::new_check_button();
                toreturn.add_object(checkbutton);
                
                let foldbutton = AppearanceData::new_fold_button();
                toreturn.add_object(foldbutton);
                
                let raisebutton = AppearanceData::new_raise_button();
                toreturn.add_object(raisebutton);    
                
                let costtocheck = AppearanceData::new_cost_to_check(debtowed);
                toreturn.add_object(costtocheck);
            }
            
            
            
        }
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        //the list of objects that can be selectable with the currently selected object
        let mut highlightedobjects = Vec::new();
        
        if let Some(selectedobject) = self.selectedobject{
            highlightedobjects = self.localgame.get_this_objects_selectable_objects(selectedobject);
            toreturn.make_object_colour( objecttype_to_objectname(selectedobject), (10.0,10.0,254.0) );
        }
        
        
        //set those objects to highlighted in the struct being returned
        for highlightedobject in highlightedobjects{
            let highlightedobjectname = objecttype_to_objectname(highlightedobject);
            toreturn.make_object_colour(highlightedobjectname, (0.0,255.0,0.0));
        }
        
        
        
        
        //highlight the list of objects up for offer
        for pieceid in &self.piecesforoffer{
            
            let objecttype = ObjectType::piece(*pieceid);
            let highlightedobjectname = objecttype_to_objectname(objecttype);
            
            toreturn.make_object_colour(highlightedobjectname, (0.0,255.0,0.0));
        }
        
        
        
        let vecofpieces: Vec<u16> = self.piecesforoffer.clone().into_iter().collect();
        let valueoffered = self.localgame.get_value_of_offered_pieces(vecofpieces);
        let valuetocheck = self.localgame.get_cost_to_check();
        
        if let Some(valueoffered) = valueoffered{
            if let Some(valuetocheck) = valuetocheck{
                
                toreturn.append_value_selected(valueoffered);
                
                
                //highlight the check button if value offered equals value to check
                if valueoffered == valuetocheck{
                    toreturn.make_object_colour("check button".to_string(),  (0.0,255.0,0.0));
                }
                
                if valueoffered > valuetocheck{
                    toreturn.make_object_colour("raise button".to_string(), (0.0,255.0,0.0));
                }
                
                if valueoffered == 0{
                    toreturn.make_object_colour("fold button".to_string(), (0.0,255.0,0.0));
                }
                
                
            }
        }
        
        
        
        
        toreturn
    }
    
    
    
    
    
    
    //if this object selectable by me
    pub fn is_object_selectable(&self, object: ObjectType) -> bool{
        
        //assume I am not in value gathering mode
        //assume i am 
        
        
        //what makes an object selectable?
        
        /*
        if its a piece owned by me
        if its a 
        
        */
        
        
    }
    
    
    //return if the object can be dragged by the player
    pub fn can_object_be_dragged(&self, object: ObjectType) -> bool{
        
        true
    }
    
    
    pub fn receive_game_update(&mut self, string: Vec<u8>){
        
        if let Ok(newgame) = bincode::deserialize::<MainGame>(&string){
            
            self.thegame = newgame;
        }
        else{
            panic!("didnt work");
        }
        
        
    }
    














    fn get_value_of_offered_pieces(&self, pieces: Vec<u16>) -> Option<u8>{
        
        self.thegame.get_value_of_offered_pieces(self.playerid, pieces)
    }
    
    fn get_cost_to_check(&self) -> Option<u8>{
        
        self.thegame.get_cost_to_check(&self.playerid)
    }
    
    //returns true if i am the owner of this object
    //OR if its an object which im allowed to select, like raise, check, deck
    //false otherwise
    fn do_i_own_object(&self, object: ObjectType) -> bool{
        
        
        if self.does_object_still_exist(object){
            
            if let ObjectType::card(cardid) = object{
                
                if let Some(ownerid)= self.thegame.get_card_owner(cardid){
                    if ownerid  == self.playerid{
                        return true;
                    }
                }
            }
            else if let ObjectType::piece(pieceid) = object{
                
                if self.playerid == self.thegame.get_board_game_object_owner(pieceid){
                    return true;
                }
            }
            else if let ObjectType::deck = object{
                return true;
            }
            else if let ObjectType::foldbutton = object{
                return true;
            }
            else if let ObjectType::raisebutton = object{
                return true;
            }
            else if let ObjectType::checkbutton = object{
                return true;
            }
            
        }
        
        
        
        return false;
        
    }
    
    //if this piece can be proposed to be offered by this player
    fn can_piece_be_offered(&self, pieceid: u16) -> bool{
        
        self.thegame.can_piece_be_offered(self.playerid, pieceid)
    }
    
    //if theres a cardgame going on
    fn is_cardgame_ongoing(&mut self) -> bool{
        
        self.thegame.is_pokergame_ongoing()
    }
    
    //gets a map of every valid player input for this given object
    //mapped by the id of the object that needs to be clicked on for it to be performed
    fn get_inputs_of_object(&self, objectid: ObjectType) -> HashMap< ObjectType, PlayerInput >{
        
        let mut toreturn = HashMap::new();
        
        
        //if the object is a piece
        if let ObjectType::piece(pieceid) = objectid{
            
            //get the actions allowed by the piece
            let actionsandobjects = self.thegame.get_actions_allowed_by_piece(pieceid);
            
            //for every action allowed, get the objectid of the board square and the piece id associated it can capture
            for (action, objectids) in actionsandobjects.1{
                
                let input = PlayerInput::pieceaction(pieceid, action);
                
                //for every object id
                for objectid in objectids{
                    
                    let objecttype;
                    
                    //if the object is a piece
                    if self.thegame.is_board_game_object_piece(objectid){
                        
                        objecttype = ObjectType::piece(objectid);
                    }
                    else if self.thegame.is_board_game_object_square(objectid){
                        
                        objecttype = ObjectType::boardsquare(objectid);
                    }
                    else{
                        panic!("apparently its neither boardsquare or piece");
                    }
                    
                    toreturn.insert( objecttype, input.clone() );
                }
                
            }
            
        }
        //if the object is a card
        else if let ObjectType::card(cardid) = objectid{
            
            //get the pieces and squares actable by the card
            let idtoinput = self.thegame.get_boardobject_actions_allowed_by_card(self.playerid, cardid);
            
            
            for (id, input) in idtoinput{
                
                if self.thegame.is_board_game_object_piece(id){
                    toreturn.insert( ObjectType::piece(id), input );
                }
                else if self.thegame.is_board_game_object_square(id){
                    toreturn.insert( ObjectType::boardsquare(id), input );
                }
                
            }
            
            
        }
        //if the object is a board square
        else if let ObjectType::boardsquare(id) = objectid{
            
            //dont do anything to fill the list to return
            //because no actions can be performed by a board square
            
        }
        
        
        toreturn
    }
    
    fn get_this_objects_selectable_objects(&self, objectid: ObjectType) -> Vec<ObjectType>{
        
        let objecttoinput = self.get_inputs_of_object(objectid);
        
        let mut toreturn = Vec::new();
        
        for (objectid, input) in objecttoinput{
            toreturn.push(objectid);
        };
        
        toreturn
        
    }
    
    //returns if this piece can be flicked or not
    fn can_piece_be_flicked(&self, pieceid: u16) -> bool{
        
        //if i own this piece
        //and its not a boardgame active
        if self.do_i_own_object( ObjectType::piece(pieceid) ){
            
            return self.thegame.get_actions_allowed_by_piece(pieceid).0;
        }
        
        return false;
    }
    
    //get the appearance of this object
    fn get_object_appearance(&mut self, objectid: ObjectType) -> AppearanceData{
        
        //if its a card
        if let ObjectType::card(cardid) = objectid{
            
            let card = self.thegame.get_card_by_id(cardid);
            
            let (field, cardposition, fieldsize) = self.thegame.where_is_card(cardid);
            
            let objectname = objecttype_to_objectname(objectid);
            
            
            let mut xpos = cardposition as f32 * 2.0;
            let ypos = 0.0;
            let zpos;
            
            let xrot = 0.0;
            let yrot = 0.0;
            let zrot = 0.0;
            
            
            if field == 1{
                zpos = -6.0;
            }
            else if field == 2{
                zpos = 6.0;
            }
            else if field == 3{
                zpos = -3.0;
                xpos += 5.5;
            }
            else if field == 4{
                zpos = 3.0;
                xpos += 5.5;
            }
            else{
                zpos = 0.0;
                xpos += 5.5;
            }
            
            
            
            let toreturn = AppearanceData::new_card( objectname, (xpos, ypos, zpos), (xrot, yrot, zrot), card );
            
            
            return toreturn ;
            
        }
        else if let ObjectType::piece(pieceid) = objectid{
            
            let position = self.thegame.get_board_game_object_translation( pieceid );
            let rotation = self.thegame.get_board_game_object_rotation( pieceid );
            let objectname = objecttype_to_objectname(objectid);
            
            let ownerid = self.thegame.get_board_game_object_owner(pieceid);
            
            let typename = self.thegame.get_piece_type_name(pieceid);
            
            //if this is a new mesh
            
            let toreturn = AppearanceData::new_piece( objectname, typename, position, rotation, ownerid );
            
            
            return toreturn;
        }
        else if let ObjectType::boardsquare(bsid) = objectid{
            
            let position = self.thegame.get_board_game_object_translation( bsid );
            let rotation = self.thegame.get_board_game_object_rotation( bsid );
            let objectname = objecttype_to_objectname(objectid);
            
            let issquarewhite = self.thegame.is_boardsquare_white(bsid);
            
            let toreturn = AppearanceData::new_boardsquare( objectname, position, rotation, issquarewhite );
            
            
            return toreturn;
        }
        else{
            panic!("why isnt the object id matching with an object of any of these types?");
        };
    }
    
    //get a list of each object in the game by id (objecttype)
    //every piece, board square, and card
    fn get_objects(&self) -> Vec<ObjectType>{
        
        let boardobjectids = self.thegame.get_board_game_object_ids();
        let cardobjectids = self.thegame.get_card_ids();
        
        let mut toreturn = Vec::new();
        
        
        for boardobjectid in boardobjectids{
            
            //get if this is a card or a boardsquare
            if self.thegame.is_board_game_object_piece(boardobjectid){
                let objectid = ObjectType::piece(boardobjectid);
                
                toreturn.push(objectid);
            }
            else if self.thegame.is_board_game_object_square(boardobjectid){
                let objectid = ObjectType::boardsquare(boardobjectid);
                
                toreturn.push(objectid);
            };
            
            
        };
        
        for cardobjectid in cardobjectids{
            let objectid = ObjectType::card(cardobjectid);
            
            toreturn.push(objectid);
        };
        
        
        
        toreturn
    }
    
    //get an objects flat position on the plane
    fn get_object_flat_plane_position(&self, objectid: ObjectType) -> (f32,f32){
        
        if let ObjectType::piece(objectid) = objectid{
            
            //get its position
            let (xpos, ypos, zpos) = self.thegame.get_board_game_object_translation(objectid);
            
            return  (xpos,zpos ) ;
            
            
        }
        
        (0.0,0.0)
    }
    
    fn get_appearance_id_of_card(card: &Card) -> u32{
        
        //giving a card of every suit and value a unique ID
        let toreturn =  4 * (card.numbervalue() -1) + card.suitvalue()  + 1;
        
        toreturn as u32
    }
    
    //get the name of this cards texture
    fn get_name_of_cards_texture(card: &Card) -> String{
        
        let cardappearanceid = LocalGameInterface::get_appearance_id_of_card(card);
        let cardappearancestring = format!("{:03}", cardappearanceid );
        "cardart/card_".to_string() + &cardappearancestring + ".jpg"
        
    }
    
    //returns whether this object exists in the game
    fn does_object_still_exist(&self, object: ObjectType) -> bool{
        
        if let ObjectType::piece(pieceid) = object{
            if self.thegame.get_board_game_object_ids().contains(&pieceid){
                return true;
            }
            else{
                return false;
            }
        }
        else if let ObjectType::card(cardid) = object{
            if self.thegame.get_card_ids().contains(&cardid){
                return true;
            }
            else{
                return false;
            }
        }
        else{
            return true ;
        };
    }
    
    
    
}









#[derive(PartialEq, Copy, Clone, Hash, Eq, Debug)]
pub enum ObjectType{
    
    card(u16),
    boardsquare(u16),
    piece(u16),
    
    deck,
    foldbutton,
    raisebutton,
    checkbutton,
    debtbutton,
    
}








//turn an object name into an object type and its ID
pub fn objectname_to_objecttype(objectname: String) -> Option<ObjectType> {
    
    
    if objectname == "deck"{
        return Some( ObjectType::deck  );
    }
    else if objectname == "raise button"{
        return Some( ObjectType::raisebutton );
    }
    else if objectname == "fold button"{
        return Some( ObjectType::foldbutton );
    }
    else if objectname == "check button"{
        
        return Some( ObjectType::checkbutton );
    }
    else if objectname == "debt button"{
        
        return Some( ObjectType::debtbutton );
    }
    //if the first character of the objects name is "P"
    else if objectname.chars().nth(0).unwrap() == 'P'{
        
        //get the rest of the name and try to convert it to an int
        let stringpieceid = objectname[1..].to_string();
        let intpieceid = stringpieceid.parse::<u16>().unwrap();
        let toreturn = ObjectType::piece(intpieceid);
        
        return Some (toreturn);
        
    }
    //if the first character of the objects name is "C"
    else if objectname.chars().nth(0).unwrap() == 'C'{
        
        //get the rest of the name and try to convert it to an int
        let stringcardid = objectname[1..].to_string();
        let intcardid = stringcardid.parse::<u16>().unwrap();
        let toreturn = ObjectType::card(intcardid);
        
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


//turn an object type into its object name
pub fn objecttype_to_objectname(inputobjecttype: ObjectType) -> String {
    
    if let ObjectType::piece(pieceid) = inputobjecttype{
        
        let toreturn = "P".to_string() + &pieceid.to_string();
        return toreturn ;
    }
    else if let ObjectType::boardsquare(boardsquareid) = inputobjecttype{
        
        let toreturn = "B".to_string() + &boardsquareid.to_string();
        return toreturn ;
    }
    else if let ObjectType::card(cardid) = inputobjecttype{
        
        let toreturn = "C".to_string() + &cardid.to_string();
        return toreturn ;
    }
    else if let ObjectType::deck = inputobjecttype{
        return "deck".to_string();
    }
    else if let ObjectType::raisebutton = inputobjecttype{
        return "raise button".to_string();
    }
    else if let ObjectType::foldbutton = inputobjecttype{
        return "fold button".to_string();
    }
    else if let ObjectType::checkbutton = inputobjecttype{
        return "check button".to_string();
    }
    else if let ObjectType::debtbutton = inputobjecttype{
        
        return "debt button".to_string();
    }
    else{
        panic!("cant convert object type to a string");
    }
    
    
}