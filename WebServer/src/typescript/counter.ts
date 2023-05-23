function httpGet2(theUrl)
{
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.open( "GET", theUrl, false ); // false for synchronous request
    xmlHttp.send( );
    return xmlHttp.responseText;
}

let root2 = document.getElementById("root");
let currentHost2 = window.location.hostname;
let currentPort2 = window.location.port;
let events2 = httpGet(`http://${currentHost}:${currentPort}/add`);
let data = document.createElement("p");
data.innerText = events2;
root2.appendChild(data);