//http://0.0.0.0:8082/index.html?port=2432&password=F38d


console.log("script working");


let baseurl = window.location.origin;

let getgamesrequesturl = "http://bbbbeeee.com/matchmaker-api/get_available_games";
let joingameurl = "http://bbbbeeee.com/matchmaker-api/join_game/";
let ccpgameurl = baseurl + "/ccpgame/"

RefreshGameList();

setInterval( function(){ RefreshGameList(); }, 6000);



function create_join_game_button(gameid, playersingame){
    
    let btn = document.createElement("BUTTON");
    btn.innerHTML = "Join Game #" + gameid + " ("+ playersingame +"/2) in game";

    let tempvalue = gameid;
    if (playersingame >= 2){
        btn.style.color = "grey";
    }
    else{

        if (playersingame == 1){
            btn.style.color = "purple";
        }
        else{
            btn.style.color = "green";
        }

        btn.onclick = function(){
            joingame(tempvalue);
        };
    }

    btn.style.width = '200px';
    btn.style.height = '40px';
    btn.style.fontWeight = '900';
    
    //so i can delete this button when updating the list of games
    btn.className = "joingamebutton";
    
    document.body.appendChild(btn);
}





function joingame(gameid) {
    
    console.log("joining game " + gameid);

    let joingamerequest = new XMLHttpRequest();
    
    joingamerequest.open("GET", joingameurl + gameid, false); // true for asynchronous

    joingamerequest.onreadystatechange = function(){
        
        //console.log( this.responseText );
        
        var gameinfo = JSON.parse( this.responseText );

        console.log(gameinfo.addressandport);
        console.log(gameinfo.gamepassword);

        let querystring = "?";
        querystring += "addressandport=" + gameinfo.addressandport;
        querystring += "&";
        querystring += "gamepassword=" + gameinfo.gamepassword;

        let fullurl = ccpgameurl + querystring;

        window.open(fullurl);

    };
    
    joingamerequest.send();
}






function RefreshGameList(){

    DeleteOldGameList();
    SetNewGameList();

    //setTimeout(() => { SetNewGameList(); }, 10);
    
}




function DeleteOldGameList(){

    //clear all previously made buttons
    var oldbuttons = document.getElementsByClassName("joingamebutton");

    for (let index = 0; index < 100; index++) {
        if (oldbuttons[0] != null){
            oldbuttons[0].remove();
        }
    }

}



function SetNewGameList(){
    
    //first, send a message to get the list of games
    let listrequest = new XMLHttpRequest();
    
    listrequest.open("GET", getgamesrequesturl, false); // true for asynchronous 
    
    listrequest.onreadystatechange = function(){
        
        console.log( this.responseText );
        
        var gamelist = JSON.parse( this.responseText );
        
        for (gameinfo of gamelist) {
            
            create_join_game_button(gameinfo[0], gameinfo[1]);
            
            var x = document.createElement("BR");
            x.className = "joingamebutton";
            document.body.appendChild(x);

        }


        if (gamelist.length == 0){

            var x = document.createElement("P");
            x.innerText = "NO AVAILABLE SERVERS RIGHT NOW";  
            x.className = "joingamebutton";
            x.style.color = "pink";
            x.style.fontSize = "60px";
            x.style.fontWeight = "1000";
            document.body.appendChild(x);

        }
        
    };
    
    listrequest.send();
}