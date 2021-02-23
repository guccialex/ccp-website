
//when matched up to be put in a game
import init, { FullGame } from './wasmfiles/wasm_builder.js';


//the baseurl
//let baserl = document.getElementById("myBase").href;

console.log(window.location.origin);
console.log(window.location.pathname);
console.log(window.location);




//run either connected to the server if the addressandport and the gamepassword
//are passed in
//or run just as the client if theyre not


const urlParams = new URLSearchParams(window.location.search);


console.log(urlParams);



if ( (urlParams.has("addressandport") === true) && (urlParams.has("gamepassword") === true) ) {
    
    let addressandport = urlParams.get("addressandport");
    let gamepassword = urlParams.get("gamepassword");
    
    run(addressandport, gamepassword);
}
else{    
    
    run_serverless()
}













async function run(websocketaddress, gamepassword) {
    
    await init();
    
    console.log("connected to game server");
    console.log(websocketaddress);
    
    
    //create a websocket connection with the server
    let socket = new WebSocket( websocketaddress );
    
    
    
    socket.onopen = function (event) {
        
        //when connected, send a message with the password
        socket.send( gamepassword );
        
    };
    
    
    
    socket.onmessage = function (event) {
        
        
        //if its a message that im connected to the game
        if (event.data == "connected to game as player 1"){

            console.log("GAME STARTING");
            
            //remove the "onmessage "event listener
            socket.onmessage = null;
            
            //start the game and give it the socket connection with the server
            start(socket, 1);
        }
        //if its a message that im connected to the game
        else if (event.data == "connected to game as player 2"){

            console.log("GAME STARTING");
            
            //remove the "onmessage "event listener
            socket.onmessage = null;
            
            //start the game and give it the socket connection with the server
            start(socket, 2 );
        }
        
        
    };
    
}





async function run_serverless() {
    
    await init();
    
    console.log("running NOT connected to server");
    
    start(null, 1);
}





async function start(socket, playerid){
    
    
    let canvas = document.getElementById("renderCanvas"); // Get the canvas element
    
    
    //canvas.style.width = "800px";
    //canvas.style.height = "400px"; 
    
    let engine = new BABYLON.Engine(canvas, true); // Generate the BABYLON 3D engine
    
    canvas.style.width = '100%';
    canvas.style.height = '100%';
    
    console.log("started");
    
    let mygame = new GameInterface(engine, socket, playerid);
    
    
    
    //if its being started with a socket, and connected to the server
    if (socket != null){
        
        //create an event listener that when a message is received, it is sent to the game
        mygame.socket.onmessage = function (event) {
            
            console.log("got message from client");
            
            mygame.get_message(event.data);
        };
    }
    
    
    
    //run the game
    rungame(mygame);
}









async function rungame(thegame) {
    
    
    //add an event listener for the mouse going up
    window.addEventListener("click", function () {
        
        thegame.mouseup();
    });
    
    //add an event for themouse going down
    window.addEventListener("pointerdown", function () {
        
        thegame.mousedown();
    });
    
    //add an event for themouse moving
    window.addEventListener("pointermove", function () {
        
        thegame.mousemove();
    });
    
    
    //run the tick function of the game 30 times per second
    thegame.gameappearance.engine.runRenderLoop(function () {
        
        thegame.tick();
    });
    
}





























//the appearance of the game state
//doesnt this also manage getting input?
class GameApperance{
    
    constructor(engine, gameinterface, playerid){
        
        //create a scene for the engine
        let scene = new BABYLON.Scene(engine);
        
        this.engine = engine;
        
        
        // This creates and positions a free camera (non-mesh)
        let camera = new BABYLON.ArcRotateCamera("camera1", 0, 0, 0, new BABYLON.Vector3(0.0,2.0,0.0), scene);
        
        //set the position of the camera, not its target tho
        if (playerid == 1){
            camera.setPosition(new BABYLON.Vector3(0, 15, -7));
        }
        else{
            camera.setPosition(new BABYLON.Vector3(0, 15, 7));
        }
        
        camera.lowerBetaLimit = 0.1;
        camera.upperBetaLimit = (Math.PI / 2) * 1.0;
        
        camera.lowerRadiusLimit = 10;
        camera.upperRadiusLimit = 30;
        
        
        //map objectname to meshstate
        //the last state of this mesh when it was updated
        this.lastmeshstate = new Map();
        
        
        //get the canvas for this engine to attach a control tos
        let canvas = engine.getRenderingCanvas();
        
        
        
        
        
        
        camera.attachControl(canvas, true);
        camera.inputs.attached["mousewheel"].wheelPrecision = 10;
        camera.inputs.attached.keyboard.detachControl();
        
        
        // This creates a light, aiming 0,1,0 - to the sky (non-mesh)
        var light = new BABYLON.HemisphericLight("light1", new BABYLON.Vector3(0, 1, 0), scene);
        light.diffuse = new BABYLON.Color3(1.0, 1.0, 1.0);
        light.specular = new BABYLON.Color3(0.0, 0.0, 0.0);
        light.intensity = 1.5;
        
        //var light = new BABYLON.DirectionalLight("DirectionalLight", new BABYLON.Vector3(0, -1, 0), scene);
        //light.intensity = 0.5;
        
        
        this.advancedTexture = BABYLON.GUI.AdvancedDynamicTexture.CreateFullscreenUI("UI");
        
        this.wonbutton = null;
        
        this.thegameinterface = gameinterface;
        
        this.scene = scene;
        
        this.camera = camera;
        
        
        
        //create the plane
        let mesh = BABYLON.MeshBuilder.CreateBox("plane", {height: 0.008, width: 100.98, depth: 100.08 }, this.scene);
        mesh.material = new BABYLON.StandardMaterial("bs_mat", this.scene);
        mesh.material.alpha = 0.00;
        mesh.position.y = 0.75;
        
        
        
        var skybox = BABYLON.Mesh.CreateBox("skybox", 100.0, this.scene);
        var skyboxMaterial = new BABYLON.StandardMaterial("skybox", this.scene);
        skyboxMaterial.backFaceCulling = false;
        skyboxMaterial.reflectionTexture = new BABYLON.CubeTexture("skybox/skybox", this.scene);
        skyboxMaterial.reflectionTexture.coordinatesMode = BABYLON.Texture.SKYBOX_MODE;
        skyboxMaterial.diffuseColor = new BABYLON.Color3(0, 0, 0);
        skyboxMaterial.specularColor = new BABYLON.Color3(0, 0, 0);
        skyboxMaterial.disableLighting = true;
        skybox.material = skyboxMaterial;
        
        
        //this.scene.freezeActiveMeshes();
        
        
        
        this.removedoverlay = false;
        
        
        
        /*
        let eemg = new BABYLON.GUI.Image("thing", "effectcards/test.png");
        eemg.width = 0.2;
        eemg.height = 0.2;
        
        
        eemg.horizontalAlignment = BABYLON.GUI.Control.HORIZONTAL_ALIGNMENT_LEFT;
        eemg.verticalAlignment = BABYLON.GUI.Control.VERTICAL_ALIGNMENT_TOP;
        
        this.advancedTexture.addControl(eemg);
        */
        
        
        
        var options = BABYLON.SceneOptimizerOptions.ModerateDegradationAllowed(30, 1000);
        
        //hardware scaling is the resolution
        //so 2.0 means rendering at half resolution if slow
        options.addOptimization(new BABYLON.HardwareScalingOptimization(0, 2.0));
        
        
        BABYLON.SceneOptimizer.OptimizeAsync(scene, options);
        
    }
    
    
    //render the scene using the appearance data
    render(appearancedata){
        
        
        
        //if there is an image overlay passed in
        if (appearancedata.overlay != null){
            
            
            //get the name of the previous image
            let oldimagename = null;
            
            if (this.imageoverlay != null){
                oldimagename = this.imageoverlay.name;
            }
            
            
            //if the image overlay has a different name than the new one
            if (appearancedata.overlay.image != oldimagename){
                
                //console.log("DRa W A DCARD");
                //console.log("the card " + appearancedata.overlay.image);
                
                
                //remove it
                if (this.imageoverlay != null){

                    this.imageoverlay.dispose();
                    this.imageoverlay = null;
                }
                
                
                let image = appearancedata.overlay.image;
                let scale = appearancedata.overlay.scale;
                //unused
                //let pos = appearancedata.overlay.position;
                
                
                
                this.imageoverlay = new BABYLON.GUI.Image(image, image);

                this.imageoverlay.width = scale;
                //this.imageoverlay.height = scale/2;

                this.imageoverlay.stretch = BABYLON.GUI.Image.STRETCH_UNIFORM;


                //this.imageoverlay.left = "-40%";
                this.imageoverlay.top = -scale/2;
                
                this.imageoverlay.horizontalAlignment = BABYLON.GUI.Control.HORIZONTAL_ALIGNMENT_LEFT;
                this.imageoverlay.verticalAlignment = BABYLON.GUI.Control.VERTICAL_ALIGNMENT_TOP;
                
                this.advancedTexture.addControl(this.imageoverlay);

                console.log("made new overlay");
                
            }
            
        }

        else{
            
            //remove it
            if (this.imageoverlay != null){
                
                console.log("REMOVING OVERLAY");
                this.imageoverlay.dispose();    
                this.imageoverlay = null;   
            }            
        }
        
        
        
        
        
        //for each object with shape data
        for (let objectdata of appearancedata.objects){
            
            
            
            
            
            
            if (objectdata.shapetype != null) {
                
                
                
                //get the name of the object
                let objectname = objectdata.name;
                
                //the old mesh that might or might not exists
                let objectmesh = this.scene.getMeshByName(objectname);
                
                
                //if a mesh already exists for this shape delete it
                if (objectmesh != null){
                    
                    objectmesh.dispose();
                    
                }
                
                
                
                let shapetype = objectdata.shapetype;
                
                if (shapetype.Cube != undefined){
                    
                    let shapedata = shapetype.Cube;
                    
                    var faceUV = new Array(6);
                    
                    //set all values to zero
                    for (var i = 0; i < 6; i++) {
                        faceUV[i] = new BABYLON.Vector4(0, 0, 0, 0);
                    }
                    
                    faceUV[4] = new BABYLON.Vector4(0, 0, 1, 1);
                    
                    
                    let options = {
                        height : shapedata[0],
                        width  : shapedata[1],
                        depth  : shapedata[2],
                        
                        faceUV: faceUV,
                    };
                    
                    BABYLON.MeshBuilder.CreateBox(objectname, options, this.scene);
                }
                else if (shapetype.Cylinder != undefined){
                    
                    let shapedata = shapetype.Cylinder;
                    
                    
                    var faceUV = new Array(6);
                    
                    //set all values to zero
                    for (var i = 0; i < 3; i++) {
                        faceUV[i] = new BABYLON.Vector4(0, 0, 0, 0);
                    }
                    
                    faceUV[2] = new BABYLON.Vector4(1, 0, 0, 1);
                    
                    
                    let options = {
                        height : shapedata[0],
                        diameter  : shapedata[1],
                        faceUV : faceUV,
                    };
                    
                    BABYLON.MeshBuilder.CreateCylinder(objectname, options, this.scene);
                }
                else if (shapetype.Circle != undefined){
                    
                    let shapedata = shapetype.Circle;
                    
                    let options = {
                        diameter : shapedata,
                    };
                    
                    BABYLON.MeshBuilder.CreateSphere(objectname, options, this.scene);
                }
                
            }
            
            
            //if that object doesnt exist still, there needs to be one made
            //so that it doesnt create an error
            
            if ( this.scene.getMeshByName(objectdata.name) == null ){
                
                console.log("needed a shapetype to make object but didnt find one");
                console.log(objectdata);
                
                
                let options = {
                    height : 1,
                    width  : 1,
                    depth  : 1,
                };
                
                BABYLON.MeshBuilder.CreateBox(objectdata.name, this.scene);
                
            }
            
            
        }
        
        
        
        
        //for each object with texture data
        for (let objectdata of appearancedata.objects){
            
            if (objectdata.texture != null) {
                
                //get the name of the object
                let objectname = objectdata.name;
                
                //the old mesh that might or might not exists
                let objectmesh = this.scene.getMeshByName(objectname);
                
                
                
                objectmesh.material = new BABYLON.StandardMaterial("bs_mat", this.scene);
                
                let colour = new BABYLON.Color3( objectdata.texture.colour[0] / 255, objectdata.texture.colour[1] / 255, objectdata.texture.colour[2] /255);
                objectmesh.material.diffuseColor = colour;
                
                
                
                //if this object has an image for its texture
                if (objectdata.texture.image != null){
                    
                    //if the object doesnt have a texture set yet
                    objectmesh.material.ambientTexture = new BABYLON.Texture(objectdata.texture.image, this.scene);
                    
                }
                
                
                
                
                
                //if this object has text
                let textdatas = objectdata.texture.texts;
                
                if (textdatas.length != 0){
                    
                    
                    
                    let texture = new BABYLON.DynamicTexture("dynamic texture", {width:textdatas[0].xsize, height:textdatas[0].ysize}, this.scene);   
                    objectmesh.material.diffuseTexture = texture;
                    objectmesh.material.useAlphaFromDiffuseTexture = true;
                    
                    
                    
                    
                    for (let textdata of textdatas){
                        
                        let text = textdata.text;
                        let font = "bold "+textdata.fontsize+"px monospace";
                        let xpos = textdata.position[0];
                        let ypos = textdata.position[1];
                        
                        objectmesh.material.diffuseTexture.drawText(text, xpos, ypos, font, "white", "transparent", true, true);   
                    }
                    
                }
                
                
                
                
                
                
            }
            
        }
        
        
        
        
        //update every objects position
        for (let objectdata of appearancedata.objects){
            
            //get the name of the object
            let objectname = objectdata.name;
            
            
            //the old mesh that might or might not exists
            let objectmesh = this.scene.getMeshByName(objectname);
            
            
            
            //if this mesh was just created, or the shape needs to updated
            objectmesh.position.x = (objectmesh.position.x * 0.5) + (objectdata.position[0] * 0.5);
            objectmesh.position.y = (objectmesh.position.y * 0.5) + (objectdata.position[1] * 0.5);
            objectmesh.position.z = (objectmesh.position.z * 0.5) + (objectdata.position[2] * 0.5);
            
            
            objectmesh.rotation.x = objectdata.rotation[0];
            objectmesh.rotation.y = objectdata.rotation[1];
            objectmesh.rotation.z = objectdata.rotation[2];
        }
        
        
        
        //the layer
        
        
        
        let objectstokeep = new Map();
        
        for (let objectdata of appearancedata.objects){
            
            objectstokeep.set(objectdata.name);
        }
        
        
        //and each object that wasn't passed in for this tick, remove it from the list of meshes
        //if its name also isnt "plane"
        for (let mesh of this.scene.meshes) {
            
            //if the objects passed to render includes the current mesh
            if (objectstokeep.has(mesh.name)) {
                //do nothing
            }
            else{
                
                if (mesh.name == "plane" || mesh.name == "myMaterial" || mesh.name == "skybox" || mesh.name == "overlay"){
                }
                else{    
                    console.log("im disposing of", mesh.name);
                    mesh.dispose();
                }
                
            }
        }
        
        
        //check if either player won
        if (appearancedata.winningplayer != null){
            
            //if it doesnt exist already yet
            if (this.wonbutton == null){
                
                var button1 = BABYLON.GUI.Button.CreateSimpleButton("wongui", "Congrats player " + appearancedata.winningplayer + " you won.");
                button1.width = "550px"
                button1.height = "200px";
                button1.color = "white";
                button1.cornerRadius = 20;
                button1.background = "green";
                
                this.wonbutton = button1;
                
                this.advancedTexture.addControl(button1);
            }
            
        }
        
        
        this.scene.render();
        
        
    }
}


function pad(n, width, z) {
    z = z || '0';
    n = n + '';
    return n.length >= width ? n : new Array(width - n.length + 1).join(z) + n;
}





//this class is called when the player creates a new game
class GameInterface{
    
    
    
    constructor(engine, socket, playerid){
        
        
        
        //create the "appearance" object for this game, giving it the scene of the engine
        this.gameappearance = new GameApperance(engine, this, playerid);
        
        this.socket = socket;
        
        //create the wasm game
        this.wasmgame = FullGame.new(playerid);
        
        
        //if an object is being dragged (if the camera movement is disabled)
        this.draggingobject = false;
        
        //what the position of the pointer is on the y=1.5 plane when i start dragging
        this.draggingstartingposition = null;
        
    }
    
    
    
    //get a websocket message from the server
    get_message(message){
        
        //console.log("receiving a message from the server", message);
        
        //let data = new Uint8Array( message.arrayBuffer());
        
        //console.log("something", data);
        
        //give the received message to the game
        this.wasmgame.get_incoming_socket_message( message );
    }
    
    
    //render the scene
    render(){
        
        //get appearance data and send it to the GameAppearance object to render
        let appearancedata = this.wasmgame.get_appearance_data();
        this.gameappearance.render(appearancedata);
        
    }
    
    
    tick() {
        
        //tick the internal game
        this.wasmgame.tick();
        
        
        //render it
        this.render();
        
        
        //get if any outgoing message is queued to be sent
        if ( this.wasmgame.is_outgoing_socket_message_queued() ){
            
            let message = this.wasmgame.pop_outgoing_socket_message();
            
            
            //if there is a socket for this game
            if (this.socket != null){
                
                console.log("im sending a websocket message");
                
                //and send them to the server
                this.socket.send( message );
            }
        }
        
        
    }
    
    
    
    //when a player clicks
    mouseup(){
        
        //reenable the cameras ability to move
        this.gameappearance.camera.inputs.attached["mousewheel"].wheelPrecision = 10;
        this.gameappearance.camera.inputs.attached["pointers"].angularSensibilityX = 1000;
        this.gameappearance.camera.inputs.attached["pointers"].angularSensibilityY = 1000;
        
        
        //not dragging any object after the mouse is lifted
        this.draggingobject = false;
        
        //tell the wasm that its mouse up
        //so it can send the flick missions if any piece is in the middle of being flicked
        this.wasmgame.mouse_up();
        
    }
    
    
    
    //when the mouse is moved
    mousemove(){
        
        //if a piece is currently being dragged, send that information to the wasmgame
        if (this.draggingobject){
            
            
            
            var objectunder = this.gameappearance.scene.pick(this.gameappearance.scene.pointerX, this.gameappearance.scene.pointerY, function(mesh) {
                
                return mesh.name != "plane" && mesh.name != "dragindicator";  // the plane and drag indicator will not be pickable
                
            });
            
            
            
            //set the position of the cursor on the plane
            var pickResult = this.gameappearance.scene.pick(this.gameappearance.scene.pointerX, this.gameappearance.scene.pointerY, function(mesh) {
                return mesh.name == "plane";  // the plane will be the only pickable thing
            });
            
            let draggingcurposition = [pickResult.pickedPoint.x, pickResult.pickedPoint.z];
            
            
            let distancedraggedx = draggingcurposition[0] - this.draggingstartingposition[0];
            let distancedraggedz = draggingcurposition[1] - this.draggingstartingposition[1];
            
            
            if (objectunder.pickedMesh ==  null){
                
                this.wasmgame.drag_selected_object(distancedraggedx, distancedraggedz, "");
                
            }
            else{
                
                this.wasmgame.drag_selected_object(distancedraggedx, distancedraggedz, objectunder.pickedMesh.name);
                
            }
            
            
        }
        
        
        
    }
    
    
    //when the mouse goes down
    mousedown(){
        
        var pickResult = this.gameappearance.scene.pick(this.gameappearance.scene.pointerX, this.gameappearance.scene.pointerY, function(mesh) {
            
            return mesh.name != "plane" && mesh.name != "dragindicator";  // the plane and drag indicator will not be pickable
        });
        
        
        console.log(pickResult.pickedMesh.name);
        
        
        //if a mesh has been clicked
        let clickedobject = pickResult.pickedMesh;
        
        
        
        
        
        //if an object was clicked on
        if (clickedobject != null) {
            
            let clickedobjectname = clickedobject.name;
            
            //if the clicked object has a name and it isnt "plane"
            if (clickedobjectname != null){
                
                
                
                
                
                //if the object is already selected, and is flickable
                if (this.wasmgame.is_object_selected_and_draggable(clickedobjectname)){
                    
                    //disable panning rotating, all camera movement basically
                    //and remporarily
                    //dont disable scrolling, it wont affect anything the player doesnt want affected when dragging
                    //this.gameappearance.camera.inputs.attached["mousewheel"].wheelPrecision = 100000;
                    this.gameappearance.camera.inputs.attached["pointers"].angularSensibilityX = 1000000;
                    this.gameappearance.camera.inputs.attached["pointers"].angularSensibilityY = 1000000;
                    
                    this.draggingobject = true;
                    
                    
                    //set the position of the cursor on the plane
                    var pickResult = this.gameappearance.scene.pick(this.gameappearance.scene.pointerX, this.gameappearance.scene.pointerY, function(mesh) {
                        return mesh.name == "plane";  // the plane will be the only pickable thing
                    });
                    
                    this.draggingstartingposition = [pickResult.pickedPoint.x, pickResult.pickedPoint.z];
                    
                    
                    
                }
                //if its not already the selected object, or is not flickable
                else{
                    this.wasmgame.mouse_down( clickedobjectname);
                }
            }
            //if the clicked object doesnt have a name, set the selected mesh to none
            else{
                this.wasmgame.mouse_down("");
            }
        }
        //if it wasnt, clear the selected object
        else{
            this.wasmgame.mouse_down("");
        }
        
        
        
    }
    
    
    
}
