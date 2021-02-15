

//http://0.0.0.0:8082/index.html?port=2432&password=F38d


console.log("updated with scriptjs working");


let baseurl = window.location.origin;


let publicgamerequest = baseurl + "/matchmaker-api/join_public_game";
let gamefilesurl = baseurl + "/ccpgame";


console.log(publicgamerequest);
console.log(gamefilesurl);
console.log(baseurl);




let xmlHttp = new XMLHttpRequest();



xmlHttp.onreadystatechange = function() {

    console.log(xmlHttp.responseText);

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
    

    //window.open(fulladdress); 
    var x = document.getElementById("myDIV");
        if (x.style.display === "none") {
          x.style.display = "block";
        } else {
          x.style.display = "none";
        }





};  


function ConnectToFoundGame() {



}




function ConnectToPublicGame() {

    document.getElementById("demo").innerHTML = "connecting to public game";

    
    xmlHttp.open("GET", publicgamerequest, true); // true for asynchronous 
    xmlHttp.send();
    
}


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



