

//http://0.0.0.0:8082/index.html?port=2432&password=F38d



let publicgamerequest = "http://35.239.40.242/matchmaker-api/join_public_game";
let matchmakerurl = "http://35.239.40.242/matchmaker-api/";
let gamefilesurl = "http://35.239.40.242/static-game-files/";


let xmlHttp = new XMLHttpRequest();



xmlHttp.onreadystatechange = function() {
    console.log(xmlHttp.responseText);

    let response = JSON.parse( xmlHttp.responseText );

    //connect to the game

    let querystring = "?";
    
    //make the query string for connecting to the game

    querystring += "addressandport=" + response.addressandport;
    querystring += "&";
    querystring += "gamepassword=" + response.gamepassword;

    let fulladdress = gamefilesurl + querystring;

    console.log(fulladdress);

    window.open(fulladdress); 

};  




function ConnectToPublicGame() {

    document.getElementById("demo").innerHTML = "connecting to public game";

    
    xmlHttp.open("GET", publicgamerequest, true); // true for asynchronous 
    xmlHttp.send();
    

    /*
    console.log("connecting to public game");
    console.log(gamefilesurl);
    */
}


function ConnectToPrivateGame() {
    document.getElementById("demo").innerHTML = "connecting to private game";

    let password = document.getElementById("gamepassword").value;

    console.log("connect to private game" + password);
}


function CreatePrivateGame() {
    document.getElementById("demo").innerHTML = "creating private game";

    console.log("creating private game");

}



