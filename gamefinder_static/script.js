

//http://0.0.0.0:8082/index.html?port=2432&password=F38d







function ConnectToPublicGame() {
    document.getElementById("demo").innerHTML = "connecting to public game";


    theUrl = "http://35.222.154.21:8000/";

    
    var xmlHttp = new XMLHttpRequest();
    
    
    xmlHttp.onreadystatechange = function() { 
        if (xmlHttp.readyState == 4 && xmlHttp.status == 200)
            callback(xmlHttp.responseText);
    }

    xmlHttp.open("GET", theUrl, true); // true for asynchronous 
    xmlHttp.send(null);
    

    console.log("connecting to public game");
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



