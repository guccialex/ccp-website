
use serde::{Serialize, Deserialize};

use std::collections::HashMap;

use physicsengine::GameEffects;

use physicsengine::CardEffect;


//a struct representing the entire state of a games physical appearance
//this struct is serialized and sent to the javascript frontend as json
#[derive(Serialize, Deserialize, Clone)]
pub struct FullAppearanceState{
    
    //the position of the camera
    
    //the visible objects
    pub objects: Vec<AppearanceData>,
    
    //if either player won
    winningplayer: Option<u8>,
    
    //the overlay
    overlay: Option<Overlay>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct Overlay{
    
    image: String,
    
    scale: f32,
    
    position: (f32,f32),
}



impl FullAppearanceState{
    
    pub fn new() -> FullAppearanceState{
        FullAppearanceState{
            objects: Vec::new(),
            
            winningplayer: None,
            
            overlay: None,
        }   
    }
    
    fn add_object(&mut self, objectappearance: AppearanceData){
        
        self.objects.push(objectappearance);
    }
    
    
    pub fn player_won(&mut self, playerid: u8){
        self.winningplayer = Some(playerid);
    }


    pub fn waiting_for_opponent(&mut self){

        self.set_overlay( "effectcards/waitingforopponent.png".to_string() , 0.15, (0.0, 0.0) );
    }
    
    
}





//the public methods to this
impl FullAppearanceState{
    
    
    
    
    pub fn set_gameobject_colour(&mut self, gameobjectname: String, colour: (u8,u8,u8)){
        
        
        //highlight every object in the list of objects that has the name specified
        for object in self.objects.iter_mut(){
            
            if gameobjectname == object.name{
                
                object.set_colour( colour);
            }
        }
    }
    
    
    //what colour to tint the object and by what amount (1.0 = 100%, 0.0 = 0%)
    pub fn tint_object_colour(&mut self, objectname: String, colour: (u8, u8, u8), mut tintamount: f32){
        
        
        for curobject in self.objects.iter_mut(){
            
            if curobject.name == objectname{
                
                curobject.tint_colour(colour, tintamount);

                return ();
            }
        }

        panic!("cannot find object {:?} to tint", objectname);
        
    }
    
    
    
    
    pub fn new_cue(&mut self, pos: (f32,f32,f32), rot: (f32,f32,f32)){
        
        let mut toadd = AppearanceData::default_object("dragindicator".to_string(), pos, rot);
        
        toadd.set_colour( (100,100,100) );
        
        toadd.set_cube( (0.2, 0.2, 1.2) );
        
        self.objects.push(toadd);
    }
    
    //remove the shape and texture from an object thats been created
    pub fn remove_shape(&mut self, objectname: String){
        
        for curobject in self.objects.iter_mut(){
            
            if curobject.name == objectname{
                
                curobject.shapetype = None;
            }
        }
    }
    
    pub fn remove_texture(&mut self, objectname: String){
        
        for curobject in self.objects.iter_mut(){
            
            if curobject.name == objectname{
                
                curobject.texture = None;
            }
        }
        
    }
    
    
    
    pub fn remove_unchanged_shapes_and_textures(&mut self, prevappearances: & HashMap<String, AppearanceData>) -> HashMap<String, AppearanceData>{
        
        
        let mut curappearances = HashMap::new();
        
        for appearancestate in & self.objects{
            
            curappearances.insert( appearancestate.name.clone(), appearancestate.clone() );
        };
        
        
        let curappearances = curappearances;
        
        
        
        for (curname, curappearance) in & curappearances{
            
            if let Some(prevappearance) = prevappearances.get( curname){
                
                if prevappearance.shapetype == curappearance.shapetype{
                    self.remove_shape( curname.clone() );
                }
                
                if prevappearance.texture == curappearance.texture{
                    
                    self.remove_texture( curname.clone() );
                }
            }
        }
        
        
        curappearances
    }
    
    pub fn new_deck(&mut self, turnstilldraw: Option<u32>){
        
        let mut toadd = AppearanceData::default_object("deck".to_string(), (-6.0,0.0,0.0), (0.0,0.0,0.0));
        
        
        toadd.set_cube( (0.6, 1.96, 1.4) );


        if let Some(tilldraw) = turnstilldraw{

            //if can be drawn from
            if tilldraw == 0{

                toadd.set_colour( (100,254,100) );            
                toadd.add_text(format!("DRAW"), (0.0,50.0), 27)

            }
            else{

                toadd.set_colour( (100,100,100) );

                if tilldraw < 10{
                    toadd.add_text(format!("{}", tilldraw), (0.0,85.0), 92)
                }
                else{
                    toadd.add_text(format!("{}", tilldraw), (0.0,70.0), 55)
                }

            }

        
        }
        else{

            toadd.set_colour( (0,0,0) );
        }
        
        self.objects.push(toadd);
    }
    
    pub fn new_timer(&mut self, playerid: u32, ticksleft: u32, currentlyturn: Option<u32>) {
        
        
        //the time left should be as minutes then seconds
        let seconds = ticksleft / 30;
        
        let minutestext = (seconds / 60).to_string();
        let secondstext = format!("{:02}", seconds % 60);
        
        let timeleft = minutestext + ":" + &secondstext;
        
        
        let position;
        let name;
        
        if playerid == 1{
            position = (-5.6,0.0,-3.0);
            name = "player".to_string() + &playerid.to_string() + "timer";
        }
        else if playerid == 2{
            position = (-5.6,0.0,3.0);
            name = "player".to_string() + &playerid.to_string() + "timer";
        }
        else{
            panic!("ahhh");
        }
        
        
        let colour;
        if let Some(_) = currentlyturn{
            colour = (0,255,0);
        }
        else{
            colour = (255,255,255);
        }
        
        
        
        
        
        
        let mut toadd = AppearanceData::default_object(name , position, (0.0,0.0,0.0));
        
        toadd.set_colour( colour );
        toadd.set_cube( (0.01, 1.2, 3.0) );
        toadd.add_text( timeleft, (0.0,50.0), 50);
        
        
        self.objects.push(toadd);
        
        
    }
    
    pub fn new_piece(&mut self, objectname: String, maybetypename: Option<String>, position: (f32,f32,f32), rotation: (f32,f32,f32), ownerid: u8){
        
        let mut toadd = AppearanceData::default_object( objectname, position, rotation);


        let texturename;
        let colour;
        
        if let Some(typename) = maybetypename{
            
            if ownerid == 1{
            
                colour = (255,255,255);
                texturename = "pieceart/".to_string() + &typename ;
            }
            else if ownerid == 2{
                
                colour = (255,255,255);
                texturename = "pieceart/b_".to_string() + &typename;
            }
            else{
                
                colour = (255,5,255);
                texturename = "pieceart/".to_string() + &typename ;
            }

            toadd.set_image( texturename );
            toadd.set_colour( colour );
        }
        
        

        toadd.set_cylinder( (0.5, 0.7) );
        
        self.objects.push(toadd);
    }
    
    
    pub fn new_game_effects(&mut self, gameeffects: &GameEffects, playerid: &u8){
        
        let effects = gameeffects.get_game_effect_names();


        let mut totalcards = 0;

        let mut yrot = 0.0;

        if playerid == &1{
            yrot = 3.14/2.0;
        }
        else if playerid == &2{
            yrot = -3.14/2.0;
        }
        
        for effect in effects{
            
            let xnumb = totalcards % 3;
            let znumb = totalcards / 3;
            
            //the position 
            let xpos = xnumb as f32 * 2.5 + 5.5;
            let ypos = 0.0 ;//+ *number as f32;
            let zpos;

            if playerid == &1{
                zpos = 4.0 - znumb as f32 * 3.7 ;//+ *number as f32;
            }
            else{
                zpos = -4.0 + znumb as f32 * 3.7 ;//+ *number as f32;
            }

            



            
            let name = format!("CardNumber{}",totalcards);
            
            let mut toadd =  AppearanceData::default_object( name , (xpos, ypos, zpos) , (0.0, yrot, 0.0) );
            
            
            toadd.set_cube( (0.1, 3.5, 2.25) );
            toadd.set_image("effectcards/".to_string() + &effect);
            
            
            self.objects.push(toadd);
            
            totalcards += 1;
        }



        
    }
    
    
    
    
    pub fn new_card_effect_display(&mut self, cardeffect: & CardEffect){
        
        self.set_overlay( "effectcards/".to_string() + &cardeffect.get_card_texture_location() , 0.15, (0.0, 0.0) );
    }
    
    
    
    
    
    
    pub fn new_boardsquare(&mut self, objectname: String, position: (f32,f32,f32), rotation: (f32,f32,f32), white: bool ){
        
        let mut toadd = AppearanceData::default_object( objectname, position, rotation );
        
        toadd.set_cube( (1.0, 1.0, 1.0) );
        
        
        if white{
            toadd.set_colour( (255,255,255) );
        }
        else{
            toadd.set_colour( (0,0,0) );
        }
        
        
        self.objects.push(toadd);
        
    }
    
    
    
    
    fn set_overlay(&mut self, image: String, scale: f32, position: (f32,f32)){
        
        self.overlay = Some(Overlay{
            
            image: image,
            
            scale: scale,
            
            position: position,
        });
        
    }
    
    
    
    
    
}





//the most complete way form of an object
//for babylon to take and display

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct AppearanceData{
    
    name: String,
    
    position: (f32,f32,f32),
    rotation: (f32,f32,f32),
    
    //the shape
    shapetype: Option<ShapeType>,
    
    //the texture
    texture: Option<Texture>,
}


//private appearance data functions
impl AppearanceData{
    
    
    fn default_object(objectname: String, position: (f32,f32,f32), rotation: (f32,f32,f32)) -> AppearanceData{
        
        let shape = CubeShape(1.0,1.0,1.0);
        
        let texture = Texture{
            colour: (100,100,100),
            image: None,
            texts: Vec::new(),
        };
        
        
        AppearanceData{
            name: objectname,
            position: position,
            rotation: rotation,
            
            shapetype: None,//ShapeType::Cube(shape),
            
            texture: None,//texture,
            
        }
    }
    
    
    
    fn set_sphere(&mut self, diameter: f32){
        
        let shape = CircleShape(diameter);
        
        self.shapetype = Some(ShapeType::Circle(shape));
    }
    
    fn set_cylinder(&mut self, dimensions: (f32,f32) ){
        
        let shape = CylinderShape(dimensions.0, dimensions.1);
        
        self.shapetype = Some(ShapeType::Cylinder(shape));
    }
    
    fn set_cube(&mut self, dimensions: (f32,f32,f32)){
        
        let shape = CubeShape(dimensions.0, dimensions.1, dimensions. 2);
        
        self.shapetype = Some(ShapeType::Cube(shape));
    }
    
    
    
    fn set_colour(&mut self, colour: (u8,u8,u8)){
        
        //if texture doesnt exist create it
        if self.texture.is_none(){
            self.texture = Some(Texture::default_texture());
        };
        
        if let Some(texture) = &mut self.texture{
            texture.colour = colour;
        };
        
    }
    
    
    fn tint_colour(&mut self, colour: (u8, u8, u8), mut tintamount: f32){
        
        //if texture doesnt exist create it
        if self.texture.is_none(){
            self.texture = Some(Texture::default_texture());
        };
        
        
        let tintingcolourfloat = (colour.0 as f32, colour.1 as f32, colour.2 as f32);
        
        //make the tint amount in the appropriate range
        if tintamount > 1.0{
            tintamount = 1.0
        }
        if tintamount < 0.0{
            tintamount = 0.0;
        }
        
        let tintinverse = 1.0 - tintamount;
        
        
        let colourfloat = (self.texture.as_ref().unwrap().colour.0 as f32,
        self.texture.as_ref().unwrap().colour.1 as f32,
        self.texture.as_ref().unwrap().colour.2 as f32);
        
        let mixedr = tintingcolourfloat.0 * tintamount + colourfloat.0 * tintinverse;
        let mixedg = tintingcolourfloat.1 * tintamount + colourfloat.1 * tintinverse;
        let mixedb = tintingcolourfloat.2 * tintamount + colourfloat.2 * tintinverse;
        
        
        
        if let Some(texture) = &mut self.texture{
            texture.colour = (mixedr as u8, mixedg as u8, mixedb as u8);
        };
        
    }
    
    fn add_text(&mut self, text: String, position: (f32,f32), fontsize: u32){
        
        //this overrides any texture set for whatever reason
        
        
        
        //if texture doesnt exist create it
        if self.texture.is_none(){
            self.texture = Some(Texture::default_texture());
        };
        
        
        let mut xsize = 100.0;
        let mut ysize = 100.0;
        
        //get the dimensions to get the size
        if let Some( ShapeType::Cube( CubeShape(x,y,z) ) ) = &self.shapetype{
            xsize = z * 50.0;
            ysize = y * 50.0;
        }
        
        
        if let Some(texture) = &mut self.texture{
            texture.texts.push(Text{
                text: text,
                position: position,
                fontsize: fontsize,
                
                xsize: xsize,
                ysize: ysize,
                
            });
        };
        
    }
    
    fn set_image(&mut self, image: String){
        
        //if texture doesnt exist create it
        if self.texture.is_none(){
            self.texture = Some(Texture::default_texture());
        };
        
        
        if let Some(texture) = &mut self.texture{
            
            texture.image = Some(image);
        };
        
    }
    
    
    
}








#[derive(Serialize, Deserialize, Clone, PartialEq)]
//#[serde(tag = "type")]
pub enum ShapeType{
    Cube(CubeShape),
    Cylinder(CylinderShape),
    Circle(CircleShape),
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct CubeShape(f32,f32,f32);

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct CylinderShape(f32,f32);

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct CircleShape(f32);




#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct Texture{
    colour: (u8,u8,u8),
    
    image: Option<String>,
    
    texts: Vec<Text>,
}


impl  Texture{
    
    fn default_texture() -> Texture{
        
        Texture{
            colour: (200,200,200),
            image: None,
            texts: Vec::new(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct Text{
    text: String,
    
    position: (f32,f32),
    
    fontsize: u32,
    
    
    xsize: f32,
    ysize: f32,
}
