

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
    
    pub fn new() -> FullAppearanceState{
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
    
    
    /*
    //add an object to display that displays X / X
    pub fn append_value_selected(&mut self, valuex: u8){
        
        self.add_object( AppearanceData::new_piece_value_offered(valuex) );
    }
    */
    
    
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


    pub fn new_cue(&mut self, pos: (f32,f32,f32), rot: (f32,f32,f32)){
        
        let mut toadd = AppearanceData::default_object("dragindicator".to_string(), pos, rot);

        toadd.set_colour( (100,100,100) );

        toadd.set_cube( (0.2, 0.2, 1.2) );
        
        self.objects.push(toadd);
    }
    
    pub fn new_deck(&mut self){

        let mut toadd = AppearanceData::default_object("deck".to_string(), (-7.0,0.0,0.0), (0.0,0.0,0.0));

        toadd.set_colour( (200,200,200) );
        toadd.set_cube( (0.6, 1.96, 1.4) );
        toadd.set_image( "cardart/cardback.jpg".to_string() );
        
        
        self.objects.push(toadd);
    }
    
    pub fn new_timer(&mut self, playerid: u32, ticksleft: u32, currentlyturn: bool) {


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






        let mut toadd = AppearanceData::default_object(name , position, (0.0,0.0,0.0));

        toadd.set_colour( (200,200,200) );
        toadd.set_cube( (0.01, 2.0, 2.0) );
        toadd.set_text( timeleft, (0.0,30.0), 30);
        
        
        self.objects.push(toadd);
        
        
    }
    
    pub fn new_piece(&mut self, objectname: String, typename: String, position: (f32,f32,f32), rotation: (f32,f32,f32), ownerid: u8){
        
        
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
        
        
        let mut toadd = AppearanceData::default_object( objectname, position, rotation);

        toadd.set_colour( colour );
        toadd.set_image( texturename );

        
        if typename == "poolball"{
            toadd.set_sphere(0.7);
        }
        else{            
            toadd.set_cylinder( (0.5, 0.7) );
        }
        
        self.objects.push(toadd);
    }
    
    pub fn new_card(&mut self, objectname: String, position: (f32,f32,f32), mut rotation: (f32,f32,f32), cardtexture: String ) {
        
        //let texturename = LocalGameInterface::get_name_of_cards_texture(&card);
        
        rotation.1 += 3.14159 / 2.0;
        
        let mut toadd = AppearanceData::default_object( objectname, position, rotation );

        toadd.set_colour( (200,200,200) );
        toadd.set_cube( (0.1, 1.96, 1.4) );
        toadd.set_image( cardtexture );
        
        
        self.objects.push(toadd);

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
    
    pub fn new_check_button(&mut self){

        let mut toadd = AppearanceData::default_object("check button".to_string(), (5.5,0.0,-6.0), (0.0,0.0,0.0));


        toadd.set_colour( (200,200,200) );


        let text = format!("check");

        toadd.set_text(text, (10.0,40.0), 20);

        toadd.set_cylinder( (0.1, 1.5) );
        
        
        self.objects.push( toadd );
    }
    
    pub fn new_fold_button(&mut self){

        let mut toadd = AppearanceData::default_object("fold button".to_string(), (7.5,0.0,-6.0), (0.0,0.0,0.0));


        toadd.set_colour( (200,200,200) );


        let text = format!("fold");

        toadd.set_text(text, (10.0,40.0), 20);

        toadd.set_cylinder( (0.1, 1.5) );
        
        
        self.objects.push( toadd );

    }
    
    pub fn new_raise_button(&mut self) {

        let mut toadd = AppearanceData::default_object("raise button".to_string(), (9.5,0.0,-6.0), (0.0,0.0,0.0));

        toadd.set_colour( (200,200,200) );


        let text = format!("raise");

        toadd.set_text(text, (10.0,40.0), 20);

        toadd.set_cylinder( (0.1, 1.5) );
        
        
        self.objects.push( toadd );
    }
    
    pub fn new_piece_value_offered(&mut self, valuex: u8) {


        let mut toadd = AppearanceData::default_object("piece value".to_string(), (-9.0,0.0,0.0), (0.0,0.0,0.0));

        toadd.set_colour( (200,200,200) );


        let text = format!("{} selected", valuex);

        toadd.set_text(text, (10.0,40.0), 20);

        toadd.set_cylinder( (0.01, 2.0) );
        
        
        self.objects.push( toadd );

        
    }
    
    pub fn new_debt_owed_button(&mut self, debt: u8) {
        

        let mut toadd = AppearanceData::default_object("debt button".to_string(), (-6.0,1.0,0.0), (0.0,0.0,0.0));

        toadd.set_colour( (200,200,200) );

        let text = format!("PAY ANTE OF {}", debt);

        toadd.set_text(text, (10.0,40.0), 10);

        toadd.set_cube( (0.01, 3.0, 3.0) );
        
        
        self.objects.push( toadd );
        
    }
    
    pub fn new_cost_to_check(&mut self, costtocheck: u8) {
        

        let mut toadd = AppearanceData::default_object( "cost to check".to_string(), (12.0, 0.0, -6.0), (0.0,0.0,0.0));

        toadd.set_colour( (200,200,200) );

        let text = format!("check {}", costtocheck);
        toadd.set_text(text, (10.0,40.0), 20);

        toadd.set_cube( (0.01, 2.0, 2.0) );
        
        
        self.objects.push( toadd );        
        
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
    shapetype: ShapeType,
    
    //the texture
    texture: Texture,
}


//private appearance data functions
impl AppearanceData{


    fn default_object(objectname: String, position: (f32,f32,f32), rotation: (f32,f32,f32)) -> AppearanceData{

        let shape = CubeShape(1.0,1.0,1.0);

        let texture = Texture{
            colour: (100,100,100),
            image: None,
            text: None,
        };


        AppearanceData{
            name: objectname,
            position: position,
            rotation: rotation,
            

            shapetype: ShapeType::Cube(shape),

            texture: texture,
        }
    }



    fn set_sphere(&mut self, diameter: f32){

        let shape = CircleShape(diameter);

        self.shapetype = ShapeType::Circle(shape);
    }

    fn set_cylinder(&mut self, dimensions: (f32,f32) ){

        let shape = CylinderShape(dimensions.0, dimensions.1);

        self.shapetype = ShapeType::Cylinder(shape);
    }

    fn set_cube(&mut self, dimensions: (f32,f32,f32)){

        let shape = CubeShape(dimensions.0, dimensions.1, dimensions. 2);

        self.shapetype = ShapeType::Cube(shape);
    }



    fn set_colour(&mut self, colour: (u8,u8,u8)){

        self.texture.colour = colour;

    }

    fn set_text(&mut self, text: String, position: (f32,f32), fontsize: u32){

        self.texture.text = Some(Text{
            text: text,
            position: position,
            fontsize: fontsize,
        });
    }

    fn set_image(&mut self, image: String){

        self.texture.image = Some(image);
    }



}























#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
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
    
    text: Option<Text>,
}


impl  Texture{

    fn default_texture() -> Texture{

        Texture{
            colour: (200,200,200),
            image: None,
            text: None,
        }


    }


}


#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct Text{
    text: String,
    
    position: (f32,f32),
    
    fontsize: u32,
}
