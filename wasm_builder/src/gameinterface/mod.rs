use std::collections::HashMap;

use physicsengine::PlayerInput;
use physicsengine::MainGame;
use physicsengine::VisibleGameObjectType;


mod appearancestate;

use appearancestate::FullAppearanceState;
use appearancestate::AppearanceData;

use super::ObjectType;
pub use super::ClientState;



//the interface the "fullgame" has with the rust chesscheckers game
pub struct LocalGameInterface{
    
    //the id of the player
    playerid: u8,
    
    //the actual rust game
    thegame: MainGame,
    
    
    //if they have the same mesh or texture as before, remove its
    prevappearance: HashMap<String, AppearanceData>,
    
}


//What methods do I want the game to interface with?
impl LocalGameInterface{
    
    
    //create a game with a certain ID
    pub fn new(playerid: u8) -> LocalGameInterface{
        
        let thegame = MainGame::new_solo_game();
        
        LocalGameInterface{
            playerid: playerid,
            thegame:thegame,
            
            prevappearance: HashMap::new(),
        }
    }
    
    //tick the local game
    pub fn tick(&mut self) {
        
        self.thegame.tick();
    }
    
    //set the state of the game according to what was received
    pub fn receive_game_update(&mut self, stringstate: String){
        
        if let Ok(_) = self.thegame.set_string_state(stringstate.clone()){
            //successfully set
        }
        else{
            panic!("the string state couldnt be serialized into the game state which was{:?}", stringstate);
        }
        
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
    
    
    
    //if this object selectable by me
    pub fn is_object_selectable(&self, object: ObjectType) -> bool{
        
        if let ObjectType::object(objectid) = object{
            
            if self.thegame.is_object_selectable( &self.playerid, &objectid){ 
                return true;
            }
        };
        return false;
    }
    
    

    
    //get the id of every object that this object targets
    fn objects_this_object_can_select(&self, objectid: ObjectType) -> Vec<ObjectType>{
        
        let objecttoinput = self.get_inputs_of_object(objectid);
        
        let mut toreturn = Vec::new();
        
        for (objectid, _) in objecttoinput{
            toreturn.push(objectid);
        };
        
        toreturn
    }

    
    
    
    //gets a map of every valid player input for this given object
    //mapped by the id of the object that needs to be clicked on for it to be performed
    fn get_inputs_of_object(&self, objectid: ObjectType) -> HashMap< ObjectType, PlayerInput >{
        
        let mut toreturn = HashMap::new();
        
        
        //if the object is a board object
        if let ObjectType::object(objectid) = objectid{
            
            //get the actions allowed by the piece
            let actionsandtargets = self.thegame.get_actions_allowed_by_piece(objectid);

            //for every action allowed, get the objectid of the board square and the piece id associated it can capture
            for (action, targetids) in actionsandtargets.1{
                
                let input = PlayerInput::pieceaction(objectid, action);
                
                //for every object id
                for targetid in targetids{
                    
                    let objecttype;
                    
                    objecttype = ObjectType::object(targetid);
                    
                    toreturn.insert( objecttype, input.clone() );
                }
            }
        }
        
        toreturn
    }
    
    


    
    pub fn get_full_appearance_state(&mut self, clientstate: &ClientState) -> FullAppearanceState{
        
        
        let ccpgamestate = self.thegame.get_visible_game_state(&self.playerid);
        
        let mut toreturn = FullAppearanceState::new();
        
        
        if let Some(winner) = ccpgamestate.isgameover{
            toreturn.player_won(winner);
        }
        
        
        toreturn.new_deck( ccpgamestate.turnsuntildrawavailable);
        
        toreturn.new_timer( 1, ccpgamestate.player1totalticksleft, ccpgamestate.playerswithactiveturns.get(&1).copied() );
        toreturn.new_timer( 2, ccpgamestate.player2totalticksleft, ccpgamestate.playerswithactiveturns.get(&2).copied() );
        
        
        
        
        
        for boardgameobject in &ccpgamestate.boardobjects{
            
            
            let gameobjectid = boardgameobject.id;

            let gameobjectname = ObjectType::object(gameobjectid).to_objectname();
            
            let mut rotation = boardgameobject.rotation;
            let position = boardgameobject.position;

            let owner = boardgameobject.owner;

            let texturelocation = boardgameobject.texturelocation.clone();

                
            //if its from player 2's perspective, rotate each piece 180 degrees 
            //and rotate each card by 180 degrees
            if self.playerid == 2{
                //i need to rotate around the objects z axis, not around the world's z axis
                //right now im just rotating around the worlds z axis but i should change that
                rotation = (boardgameobject.rotation.0 , boardgameobject.rotation.1 + 3.1416, boardgameobject.rotation.2);
            }
            
            
            if  VisibleGameObjectType::Piece == boardgameobject.objecttype{

                toreturn.new_piece( gameobjectname.clone(), texturelocation, position, rotation, owner.unwrap() );
            }
            else if let VisibleGameObjectType::Square(iswhite) = boardgameobject.objecttype{

                toreturn.new_boardsquare( gameobjectname.clone(), position, rotation, iswhite );
            }
            
            
            //tint every object on a mission blue
            if boardgameobject.isonmission{
                
                toreturn.tint_object_colour(gameobjectname.clone(), (0, 0, 0), 0.5);
                toreturn.tint_object_colour(gameobjectname, (250, 0, 250), 0.4);
            }
            
        };
        
        
        
        
        //if an object is selected
        if let Some(selectedgameobject) = clientstate.selectedobject{
            
            //make the selected game object yellow
            toreturn.set_gameobject_colour( selectedgameobject.to_objectname(), (10,10,254) );
            
            let highlightedobjects = self.objects_this_object_can_select(selectedgameobject);
            
            //make those highlighted objects green
            for highlightedobject in highlightedobjects.clone(){

                //panic!("highlighted objects {:?}", highlightedobjects);
                
                let highlightedobjectname = highlightedobject.to_objectname();
                toreturn.tint_object_colour(highlightedobjectname, (0,255,0), 0.65);

                //toreturn.set_gameobject_colour(highlightedobjectname, (0,255,0));
                
            }
        }
        
        
        
        toreturn.new_game_effects( &ccpgamestate.gameeffects, &self.playerid );

        
        
        //display the last card effect played 10 seconds ago
        //for cardeffect in &ccpgamestate.eff
        
        if let Some( effect ) = &ccpgamestate.lastcardeffect{
            
            toreturn.new_card_effect_display(effect);
        }
        
        
        
        
        self.prevappearance = toreturn.remove_unchanged_shapes_and_textures( &self.prevappearance );
        toreturn
        
    }
    
    
}





