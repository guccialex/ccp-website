

//alright. I hate thinking about javascript
//and I hate reading the docs on babylon js
//theyre just so much more messy, unclear than anything in rust
//and like
//every object has at least 100 methods for some reason
//so im going to be thinking about 



//how
//do i represent a state of the appearances of all the objects in a scene


use serde::{Serialize, Deserialize};



//a struct representing the entire state of a games physical appearance
#[derive(Serialize, Deserialize, Clone)]
pub struct FullAppearanceState{

    //the position of the camera
    
    //the visible objects
    objects: Vec<AppearanceData>,
    
}

impl FullAppearanceState{
    
    fn new() -> FullAppearanceState{
        FullAppearanceState{
            objects: Vec::new(),
        }   
    }
    
    fn add_object(&mut self, objectappearance: AppearanceData){
        
        self.objects.push(objectappearance);
    }
    
    
    /*
    pub fn make_object_colour(&mut self, objectname: String, colour: (f32,f32,f32)){
        
        for curobject in self.objects.iter_mut(){
            
            if curobject.name == objectname{
                
                let unmixedcolourfloat = colour;
                let colourfloat = (curobject.texture.colour.0 as f32, curobject.texture.colour.1 as f32, curobject.texture.colour.2 as f32);
                
                let mixedr = unmixedcolourfloat.0 * 0.8 + colourfloat.0 * 0.2;
                let mixedg = unmixedcolourfloat.1 * 0.8 + colourfloat.1 * 0.2;
                let mixedb = unmixedcolourfloat.2 * 0.8 + colourfloat.2 * 0.2;
                
                //make its colour closer to green
                curobject.texture.colour = (mixedr as u8, mixedg as u8, mixedb as u8);
                
            }
        }
    }
    */
    
    
    //add an object to display that displays X / X
    pub fn append_value_selected(&mut self, valuex: u8){
        
        self.add_object( AppearanceData::new_piece_value_offered(valuex) );
    }
    
    
}





impl AppearanceData{
    
    pub fn new_cue(pos: (f32,f32,f32), rot: (f32,f32,f32)) -> AppearanceData{
        
        
        let texture = Texture{
            
            colour: (200,200,200),
            image: None,
            text: None,
        };
        
        
        let shape = CubeShape{
            dimensions:  (0.2, 0.2, 1.2),
        };
        
        let shapetype = ShapeType::Cube(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: pos,
            
            rotation: rot,
            
        };
        
        let appearancedata = AppearanceData{
            
            name: "dragindicator".to_string(),
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
    }
    
    
    pub fn new_deck() -> AppearanceData{
        
        let imagename = "cardart/cardback.jpg".to_string();
        
        
        
        let texture = Texture{
            
            colour: (200,200,200),
            image: Some(imagename),
            text: None,
        };
        
        
        let shape = CubeShape{
            dimensions: (0.6, 1.96, 1.4),
        };
        
        let shapetype = ShapeType::Cube(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: (-7.0,0.0,0.0),
            
            rotation: (0.0,0.0,0.0),
            
        };
        
        let appearancedata = AppearanceData{
            
            name: "deck".to_string(),
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
    }
    
    
    pub fn new_timer(playerid: u32, ticksleft: u32, currentlyturn: bool) -> AppearanceData{
        
        
        //the time left should be as minutes then seconds
        let seconds = ticksleft / 30;
        
        let minutestext = (seconds / 60).to_string();
        let secondstext = format!("{:02}", seconds % 60);
        
        
        let timeleft = minutestext + ":" + &secondstext;
        
        
        let position;
        let name;
        
        if playerid == 1{
            position = (-7.0,0.0,-3.0);
            name = "player".to_string() + &playerid.to_string() + "timer";
        }
        else if playerid == 2{
            position = (-7.0,0.0,3.0);
            name = "player".to_string() + &playerid.to_string() + "timer";
        }
        else{
            panic!("ahhh");
        }
        
        
        let colour;
        
        if currentlyturn{
            colour = (0,255,0);
        }
        else{
            colour = (255,255,255);
        }
        
        
        let text = Text{
            fontsize: 30,
            position: (0.0,30.0),
            text: timeleft,
        };
        
        let texture = Texture{
            
            colour: colour,
            image: None,
            text: Some(text),
        };
        
        
        let shape = CubeShape{
            dimensions: (0.01, 2.0, 2.0),
        };
        
        let shapetype = ShapeType::Cube(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: position,
            
            rotation: (0.0,0.0,0.0),
            
        };
        
        let appearancedata = AppearanceData{
            
            name: name,
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
        
    }
    
    pub fn new_piece(objectname: String, typename: String ,position: (f32,f32,f32), rotation: (f32,f32,f32), ownerid: u8) -> AppearanceData{
        
        let texturename;
        let colour;
        
        if ownerid == 1{
            
            colour = (255,255,255);
            texturename = "pieceart/".to_string() + &typename + &".png";
            
        }
        else if ownerid == 2{
            
            colour = (255,255,255);
            texturename = "pieceart/b_".to_string() + &typename + &".png";
            
        }
        else{
            
            colour = (255,5,255);
            texturename = "pieceart/".to_string() + &typename + &".png";
            
        }
        
        let shapetype;
        
        if typename == "poolball"{
            
            let shape = CircleShape{
                diameter: 0.7,
            };
            
            shapetype = ShapeType::Circle(shape);
            
        }
        else{
            
            let shape = CylinderShape{
                dimensions: (0.5, 0.7),
            };
            
            shapetype = ShapeType::Cylinder(shape);
            
        }
        
        
        
        let texture = Texture{
            colour: colour,
            image: Some(texturename),
            text: None,
        };
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: position,
            
            rotation: rotation,
            
        };
        
        let appearancedata = AppearanceData{
            
            name: objectname,
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
        
        
    }
    
    pub fn new_card(name: String, position: (f32,f32,f32), mut rotation: (f32,f32,f32), card: Card ) -> AppearanceData{
        
        let texturename = LocalGameInterface::get_name_of_cards_texture(&card);
        
        
        rotation.1 += 3.14159 / 2.0;
        
        
        let texture = Texture{
            colour: (200,200,200),
            image: Some(texturename),
            text: None,
        };
        
        
        let shape = CubeShape{
            dimensions: (0.1, 1.96, 1.4),
        };
        
        let shapetype = ShapeType::Cube(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: position,
            
            rotation: rotation,
            
        };
        
        let appearancedata = AppearanceData{
            
            name: name,
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
        
    }
    
    
    pub fn new_boardsquare(name: String, position: (f32,f32,f32), rotation: (f32,f32,f32), white: bool ) -> AppearanceData{
        
        
        let colour;
        
        if white{
            colour = (255,255,255);
        }
        else{
            colour = (0,0,0);
        }
        
        
        let texture = Texture{
            colour: colour,
            image: None,
            text: None,
        };
        
        
        let shape = CubeShape{
            dimensions: (1.0, 1.0, 1.0),
        };
        
        let shapetype = ShapeType::Cube(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: position,
            
            rotation: rotation,
            
        };
        
        let appearancedata = AppearanceData{
            
            name: name,
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
        
    }
    
    
    pub fn new_check_button() -> AppearanceData{
        
        let text = format!("check");
        
        let text = Text{
            fontsize: 20,
            position: (10.0,40.0),
            text: text,
        };
        
        let texture = Texture{
            
            colour: (200,200,200),
            image: None,
            text: Some(text),
        };
        
        
        let shape = CylinderShape{
            dimensions: (0.1, 1.5),
        };
        
        let shapetype = ShapeType::Cylinder(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: (5.5,0.0,-6.0),
            
            rotation: (0.0,0.0,0.0),
            
        };
        
        let appearancedata = AppearanceData{
            
            name: "check button".to_string(),
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
    }
    
    
    pub fn new_fold_button() -> AppearanceData{
        
        let text = format!("fold");
        
        let text = Text{
            fontsize: 20,
            position: (10.0,40.0),
            text: text,
        };
        
        let texture = Texture{
            
            colour: (200,200,200),
            image: None,
            text: Some(text),
        };
        
        
        let shape = CylinderShape{
            dimensions: (0.1, 1.5),
        };
        
        let shapetype = ShapeType::Cylinder(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: (7.5,0.0,-6.0),
            
            rotation: (0.0,0.0,0.0),
            
        };
        
        let appearancedata = AppearanceData{
            
            name: "fold button".to_string(),
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
    }
    
    
    pub fn new_raise_button() -> AppearanceData{
        
        let text = format!("raise");
        
        let text = Text{
            fontsize: 20,
            position: (10.0,40.0),
            text: text,
        };
        
        let texture = Texture{
            
            colour: (200,200,200),
            image: None,
            text: Some(text),
        };
        
        
        let shape = CylinderShape{
            dimensions: (0.1, 1.5),
        };
        
        let shapetype = ShapeType::Cylinder(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: (9.5,0.0,-6.0),
            
            rotation: (0.0,0.0,0.0),
            
        };
        
        let appearancedata = AppearanceData{
            
            name: "raise button".to_string(),
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
        
    }
    
    pub fn new_piece_value_offered(valuex: u8) -> AppearanceData{
        
        
        let text = format!("{} selected", valuex);
        
        let text = Text{
            fontsize: 20,
            position: (10.0,40.0),
            text: text,
        };
        
        let texture = Texture{
            
            colour: (200,200,200),
            image: None,
            text: Some(text),
        };
        
        
        let shape = CylinderShape{
            dimensions: (0.01, 2.0),
        };
        
        let shapetype = ShapeType::Cylinder(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: (-9.0,0.0,0.0),
            
            rotation: (0.0,0.0,0.0),
            
        };
        
        let appearancedata = AppearanceData{
            
            name: "piece value".to_string(),
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
        
    }
    
    pub fn new_debt_owed_button(debt: u8) -> AppearanceData{
        
        let text = format!("PAY ANTE OF {}", debt);
        
        let text = Text{
            fontsize: 10,
            position: (10.0,40.0),
            text: text,
        };
        
        let texture = Texture{
            
            colour: (200,200,200),
            image: None,
            text: Some(text),
        };
        
        
        let shape = CubeShape{
            dimensions: (0.01, 3.0, 3.0),
        };
        
        let shapetype = ShapeType::Cube(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: (-6.0,1.0,0.0),
            
            rotation: (0.0,0.0,0.0),
            
        };
        
        let appearancedata = AppearanceData{
            
            name: "debt button".to_string(),
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
        
    }
    
    pub fn new_cost_to_check(costtocheck: u8) -> AppearanceData{
        
        let text = format!("check {}", costtocheck);
        
        let text = Text{
            fontsize: 20,
            position: (10.0,40.0),
            text: text,
        };
        
        let texture = Texture{
            
            colour: (200,200,200),
            image: None,
            text: Some(text),
        };
        
        
        let shape = CubeShape{
            dimensions: (0.0, 2.0, 2.0),
        };
        
        let shapetype = ShapeType::Cube(shape);
        
        
        let shape = Shape{
            
            shapetype: shapetype,
            
            position: (12.0, 0.0, -6.0),
            
            rotation: (0.0,0.0,0.0),
        };
        
        let appearancedata = AppearanceData{
            
            name: "cost to check".to_string(),
            
            shape: shape,
            
            texture: texture,
        };
        
        
        appearancedata
        
        
        
    }
    
}





















//the most complete way form of an object
//for babylon to take and display

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct AppearanceData{
    
    name: String,
    
    //the shape
    shape: Shape,
    
    //the texture
    texture: Texture,
    
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Shape{
    
    shapetype: ShapeType,
    
    position: (f32,f32,f32),
    rotation: (f32,f32,f32),
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ShapeType{
    Cube(CubeShape),
    Cylinder(CylinderShape),
    Circle(CircleShape),
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct CubeShape{
    
    dimensions: (f32,f32,f32),
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct CylinderShape{
    
    dimensions: (f32,f32),
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct CircleShape{
    diameter: f32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct Texture{
    
    colour: (u8,u8,u8),
    
    image: Option<String>,
    
    text: Option<Text>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct Text{
    
    text: String,
    
    position: (f32,f32),
    
    fontsize: u32,
    
}