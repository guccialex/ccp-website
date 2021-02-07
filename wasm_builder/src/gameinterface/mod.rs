


use physicsengine::PlayerInput;
use physicsengine::Card;

use std::collections::HashMap;

use physicsengine::MainGame;

use physicsengine::PieceAction;
use physicsengine::BlackJackAction;
use physicsengine::PokerAction;
use physicsengine::CardAction;

//use std::collections::HashSet;



mod appearancestate;


use appearancestate::FullAppearanceState;
use appearancestate::AppearanceData;


use super::ObjectType;
pub use super::ClientState;

use std::collections::HashSet;



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
            
            return self.try_to_perform_input( playerinput.clone() );
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
    pub fn try_to_check(&mut self, pieces: &HashSet<u16>) -> Option<String>{
        
        let pieces: Vec<u16> = pieces.clone().into_iter().collect();
        
        let action = PokerAction::check(pieces);
        let input = PlayerInput::pokeraction(action);
        
        
        return self.try_to_perform_input(input);
    }
    pub fn try_to_raise(&mut self, pieces: &HashSet<u16>) -> Option<String>{
        
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
    pub fn try_to_settle_debt(&mut self, pieces: &HashSet<u16>) -> Option<String>{
        
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
    
    
    
    
    
    
    pub fn get_full_appearance_state(&mut self, clientstate: &ClientState) -> FullAppearanceState{
        
        
        let mut toreturn = FullAppearanceState::new();
        
        
        
        
        
        toreturn.new_deck();
        
        //add the timer for the player and the opponent
        {
            let player1totaltimeleft = self.thegame.get_players_total_ticks_left(1);
            let iscurrentlyturn = self.thegame.get_players_turn_ticks_left(1) > 0;
            toreturn.new_timer(1, player1totaltimeleft, iscurrentlyturn);
            
            
            let player2totaltimeleft = self.thegame.get_players_total_ticks_left(2);
            let iscurrentlyturn = self.thegame.get_players_turn_ticks_left(2) > 0;
            toreturn.new_timer(2, player2totaltimeleft, iscurrentlyturn);
        }        
        
        
        
        
        
        let boardobjectids = self.thegame.get_board_game_object_ids();
        let cardobjectids = self.thegame.get_card_ids();
        
        
        
        //each board objects
        for boardobjectid in boardobjectids{
            
            
            let position = self.thegame.get_board_game_object_translation( boardobjectid );
            let rotation = self.thegame.get_board_game_object_rotation( boardobjectid );
            let ownerid = self.thegame.get_board_game_object_owner(boardobjectid);
            
            
            
            
            let gameobjectid;
            let gameobjectname;
            
            //get what type of board object it is
            
            if self.thegame.is_board_game_object_piece(boardobjectid){
                gameobjectid = ObjectType::piece(boardobjectid);
                gameobjectname = gameobjectid.to_objectname();
            }
            else if self.thegame.is_board_game_object_square(boardobjectid){
                gameobjectid = ObjectType::boardsquare(boardobjectid);
                gameobjectname = gameobjectid.to_objectname();
            }
            else{
                panic!("What else could it be?");
            }
            
            
            
            if let ObjectType::piece(_) = gameobjectid{
                
                let piecetypename = self.thegame.get_piece_type_name( boardobjectid );    
                toreturn.new_piece( gameobjectname, piecetypename, position, rotation, ownerid );
                
            }
            else if let ObjectType::boardsquare(_) = gameobjectid{
                
                let issquarewhite = self.thegame.is_boardsquare_white( boardobjectid );
                toreturn.new_boardsquare( gameobjectname, position, rotation, issquarewhite );
            }
            
            
        }
        
        
        //each card object
        for cardobjectid in cardobjectids{
            
            
            let card = self.thegame.get_card_by_id(cardobjectid);
            
            let cardtexture = LocalGameInterface::get_name_of_cards_texture(&card);
            
            
            let (field, cardposition, fieldsize) = self.thegame.where_is_card(cardobjectid);
            
            
            let gameobjectid = ObjectType::card(cardobjectid);
            
            let gameobjectname = gameobjectid.to_objectname();
            
            
            
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
            
            
            let pos = (xpos, ypos, zpos);
            let rot = (xrot, yrot, zrot);
            
            
            
            toreturn.new_card( gameobjectname, pos, rot, cardtexture );            
        }
        
        
        
        
        
        
        //change the colour of every piece offered
        for pieceid in &clientstate.piecesforoffer{
            
            let objecttype = ObjectType::piece(*pieceid);
            let highlightedobjectname = objecttype.to_objectname();
            
            toreturn.set_gameobject_colour(highlightedobjectname, (0,255,0));
        }
        
        
        
        //if an object is selected
        if let Some(selectedgameobject) = clientstate.selectedobject{
            
            
            //make the selected game object yellow
            toreturn.set_gameobject_colour( selectedgameobject.to_objectname(), (10,10,254) );
            
            
            
            let highlightedobjects = self.get_this_objects_selectable_objects(selectedgameobject);
            
            //make those highlighted objects green
            for highlightedobject in highlightedobjects{
                
                let highlightedobjectname = highlightedobject.to_objectname();
                toreturn.set_gameobject_colour(highlightedobjectname, (0,255,0));
            }
        }
        
        
        
        
        
        let debtowed = self.thegame.get_debt_of_player(&self.playerid);
        
        if debtowed != 0{
            toreturn.new_debt_owed_button(debtowed);            
        }
        else{
            
            //if theres a poker game going on
            //give the check, fold and raise buttons
            if self.thegame.is_pokergame_ongoing() {
                
                toreturn.new_check_button();
                
                toreturn.new_fold_button();
                
                toreturn.new_raise_button();
                
                toreturn.new_cost_to_check(debtowed);
            }
            
        }
        
        
        
        
        
        let vecofpieces: Vec<u16> = clientstate.piecesforoffer.clone().into_iter().collect();
        
        let valueoffered = self.thegame.get_value_of_offered_pieces(self.playerid, vecofpieces);
        
        let valuetocheck = self.thegame.get_cost_to_check(&self.playerid);
        
        
        if let Some(valueoffered) = valueoffered{
            if let Some(valuetocheck) = valuetocheck{
                
                toreturn.new_piece_value_offered(valueoffered);
                
                
                
                
                /*
                this bit kinda contains game logic that I should really have in the shared backend/client game code
                
                i guess separate exposed functions for:
                can player check? can player raise? can player fold?
                that return bools
                */
                
                
                //highlight the check button if value offered equals value to check
                if valueoffered == valuetocheck{
                    toreturn.set_gameobject_colour("check button".to_string(),  (0,255,0));
                }
                
                //highlight the raise button if the value offered is greater than the value to check
                if valueoffered > valuetocheck{
                    toreturn.set_gameobject_colour("raise button".to_string(), (0,255,0));
                }
                
                //highlight the fold button if the value offered is 0
                if valueoffered == 0{
                    toreturn.set_gameobject_colour("fold button".to_string(), (0,255,0));
                }
                
                
            }
        }
        
        
        
        //if theres an object being dragged
        //add the drag indicator to be drawn
        if let Some(dragged) = &clientstate.dragged{
            
            
            //if there is a selectedobject and it is a piece
            if let Some( ObjectType::piece(_) ) = clientstate.selectedobject{
                
                let (reldistx, reldisty) = dragged.relativepos;

                //get the position of the selected piece
                let selectedposition = self.get_object_flat_plane_position( clientstate.selectedobject.unwrap() );
                
                //get the position and rotation of the cue
                let (position, rotation) = get_position_and_rotation_of_cue_indicator(selectedposition, reldistx, reldisty);

                //add the cue to the objects to be rendered
                toreturn.new_cue(position, rotation);
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
        true
        
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
    pub fn can_piece_be_offered(&self, pieceid: u16) -> bool{
        
        self.thegame.can_piece_be_offered(self.playerid, pieceid)
    }
    
    //if theres a cardgame going on
    pub fn is_cardgame_ongoing(&mut self) -> bool{
        
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
