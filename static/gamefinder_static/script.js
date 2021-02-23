//http://0.0.0.0:8082/index.html?port=2432&password=F38d



console.log("updated with scriptjs working");

let baseurl = window.location.origin;



//first, send a message to get the list of games
let listrequest = new XMLHttpRequest();
listrequest.open("GET", baseurl+ "/get_available_games", true); // true for asynchronous 

listrequest.onload() = () => {

    console.log( listrequest.response );

    var gamelist = JSON.parse( listrequest.response );

    for (gameinfo of gamelist) {
    
        let btn = document.createElement( "BUTTON");

        btn.innerHTML = JSON.stringify( gameinfo );
        
        document.body.appendChild(btn);
    }

};

listrequest.send();



/*
//let publicgamerequest = baseurl + "/matchmaker-api/join_public_game";
let gamefilesurl = baseurl + "/ccpgame";


console.log(publicgamerequest);
console.log(gamefilesurl);
console.log(baseurl);


let foundgame = "";



let xmlHttp = new XMLHttpRequest();
xmlHttp.open("GET", baseurl+ "/get_available_games", true);



xmlHttp.onreadystatechange = function() {

    console.log(xmlHttp.responseText);

    document.getElementById("demo").innerHTML = "connection failed :(";


    let response = JSON.parse( xmlHttp.responseText );

    //connect to the game
    let querystring = "/?";
    
    //make the query string for connecting to the game
    querystring += "addressandport=" + response.addressandport;
    querystring += "&";
    querystring += "gamepassword=" + response.gamepassword;

    let fulladdress = gamefilesurl + querystring;

    console.log(fulladdress);


    //change the "connect to public game" button to green
    //and then when you click it again it sends you to the game
    
    foundgame = fulladdress;



    document.getElementById("demo").innerHTML = "";

    var x = document.getElementById("GameFound");
    x.style.display = "block";

};  





function ConnectToFoundGame() {

    window.open(foundgame);

}




function ConnectToPublicGame() {

    document.getElementById("demo").innerHTML = "connecting to public game";

    
    xmlHttp.open("GET", publicgamerequest, true); // true for asynchronous 
    xmlHttp.send();
    
}
*/



/*

    <button onclick="CreatePrivateGame()">create a private game</button>
    <br>
    <br>
    <input type="text" id="gamepassword" value="password">
    <button onclick="ConnectToPrivateGame()">join a private game</button>
    <br>


function ConnectToPrivateGame() {
    document.getElementById("demo").innerHTML = "connecting to private game";

    let password = document.getElementById("gamepassword").value;
    
    xmlHttp.open("GET", publicgamerequest, true); // true for asynchronous 
    xmlHttp.send();
}


function CreatePrivateGame() {
    document.getElementById("demo").innerHTML = "creating private game";

    xmlHttp.open("GET", publicgamerequest, true); // true for asynchronous 
    xmlHttp.send();
}


*/
