


use physicsengine::PlayerInput;
use physicsengine::Card;

use std::collections::HashMap;

use physicsengine::MainGame;

use physicsengine::PieceAction;

use physicsengine::VisibleGameObjectType;
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
    
    
    //the appearance of each 
    prevvisiblegameobject: HashMap< ObjectType, VisibleGameObjectType>,
    
    
    //if they have the same mesh or texture as before, remove its
    prevappearance: HashMap<String, AppearanceData>,
    
}


//What methods do I want the game to interface with?
impl LocalGameInterface{
    
    
    //create a game with a certain ID
    pub fn new(playerid: u8) -> LocalGameInterface{
        
        
        let thegame = MainGame::new_two_player();
        
        LocalGameInterface{
            
            playerid: playerid,
            thegame:thegame,
            
            prevvisiblegameobject: HashMap::new(),
            
            prevappearance: HashMap::new(),
            
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
        
        
        
        let objecttoinput = self.get_inputs_of_object(object1);
        
        //if there is a player input that lets object1 perform some action on object 2
        if let Some(playerinput) = objecttoinput.get(&object2){
            
            //panic!( "input sent {:?}", playerinput.clone()  );
            
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
    pub fn try_to_draw_card(&mut self) -> Option<String>{
        
        let input = PlayerInput::drawcard;
        
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
        
        
        let ccpgamestate = self.thegame.get_visible_game_state(&self.playerid);
        
        let mut toreturn = FullAppearanceState::new();
        
        
        if let Some(winner) = ccpgamestate.isgameover{
            toreturn.player_won(winner);
        }
        
        
        toreturn.new_deck( ccpgamestate.drawactionvalid );
        
        toreturn.new_timer( 1, ccpgamestate.player1totalticksleft, ccpgamestate.playerswithactiveturns.contains(&1) );
        toreturn.new_timer( 2, ccpgamestate.player2totalticksleft, ccpgamestate.playerswithactiveturns.contains(&2) );
        
        
        
        
        
        for boardgameobject in &ccpgamestate.boardobjects{
            
            let gameobjectid;
            
            //if the texture is the same as the last
            
            if let physicsengine::VisibleGameObjectType::Piece(_) = &boardgameobject.objecttype{
                gameobjectid = ObjectType::piece( boardgameobject.id );            
            }
            else if let physicsengine::VisibleGameObjectType::Square(_) = &boardgameobject.objecttype{
                gameobjectid = ObjectType::boardsquare( boardgameobject.id );
            }
            else{
                panic!("not a piece or square. huh?");
            }
            
            
            let gameobjectname = gameobjectid.to_objectname();
            
            
            
            if let physicsengine::VisibleGameObjectType::Piece(pieceobject) = &boardgameobject.objecttype{
                
                toreturn.new_piece( gameobjectname, pieceobject.typename.clone(), boardgameobject.position, boardgameobject.rotation, pieceobject.owner );
            }
            else if let physicsengine::VisibleGameObjectType::Square(squareobject) = &boardgameobject.objecttype{
                
                toreturn.new_boardsquare( gameobjectname, boardgameobject.position, boardgameobject.rotation, squareobject.iswhite );
            };
            
            
        };
        
        
        
        //if theres an object being dragged
        //add the drag indicator to be drawn
        if let Some(dragged) = &clientstate.dragged{
            
            
            //if there is a selectedobject and it is a piece
            if let Some( ObjectType::piece(pieceid) ) = clientstate.selectedobject{
                
                let (reldistx, reldisty) = dragged.relativepos;
                
                //get the position of the selected piece
                
                if let Some( (xpos, zpos) ) = ccpgamestate.get_piece_plane_position( pieceid ){
                    
                    //get the position and rotation of the cue
                    let (position, rotation) = get_position_and_rotation_of_cue_indicator( (xpos, zpos) , reldistx, reldisty);
                    
                    //add the cue to the objects to be rendered
                    toreturn.new_cue(position, rotation);
                    
                    
                };
                
            }
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
        
        
        
        self.prevappearance = toreturn.remove_unchanged_shapes_and_textures( &self.prevappearance );
        
        toreturn
        
    }
    
    
    
    
    
    
    //if this object selectable by me
    pub fn is_object_selectable(&self, object: ObjectType) -> bool{
        
        if let ObjectType::piece(pieceid) = object{

            let owner = self.thegame.get_board_game_object_owner(pieceid);

            if owner == Some(self.playerid){

                return true;
            }

        };
        
        return false;
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
    
    
    
    
    
    
    
    
    

    
    
    //gets a map of every valid player input for this given object
    //mapped by the id of the object that needs to be clicked on for it to be performed
    fn get_inputs_of_object(&self, objectid: ObjectType) -> HashMap< ObjectType, PlayerInput >{
        
        let mut toreturn = HashMap::new();
        
        
        //if the object is a piece
        if let ObjectType::piece(pieceid) = objectid{
            
            //get the actions allowed by the piece
            let actionsandobjects = self.thegame.get_actions_allowed_by_piece(pieceid);
            
            //panic!("actions allowed {:?}", actionsandobjects);
            
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
        //if the object is a board square
        else if let ObjectType::boardsquare(id) = objectid{
            
            //dont do anything to fill the list to return
            //because no actions can be performed by a board square
            
        }
        
        
        toreturn
    }
    
    
    /*
    //returns if this piece can be flicked or not
    fn can_piece_be_flicked(&self, pieceid: u16) -> bool{
        
        //if i own this piece
        //and its not a boardgame active
        if self.do_i_own_object( ObjectType::piece(pieceid) ){
            
            return self.thegame.get_actions_allowed_by_piece(pieceid).0;
        }
        
        return false;
    }
    */
    
    
    
    
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
        else{
            return true ;
        };
    }
    
    
    
    
    fn get_this_objects_selectable_objects(&self, objectid: ObjectType) -> Vec<ObjectType>{
        
        let objecttoinput = self.get_inputs_of_object(objectid);
        
        let mut toreturn = Vec::new();
        
        for (objectid, input) in objecttoinput{
            toreturn.push(objectid);
        };
        
        toreturn
    }
    
    //this should be moved into the "card" class methods
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
