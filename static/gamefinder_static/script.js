

//http://0.0.0.0:8082/index.html?port=2432&password=F38d


console.log("updated with scriptjs working");

//the base url
console.log(location.href);


let baseurl = location.href;




let publicgamerequest = baseurl + "/matchmaker-api/join_public_game";
let gamefilesurl = baseurl + "/static-game-files/";


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



