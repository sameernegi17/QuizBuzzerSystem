function httpGet(theUrl)
{
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.open( "GET", theUrl, false ); // false for synchronous request
    xmlHttp.send( );
    console.log(JSON.parse(xmlHttp.responseText));
    return xmlHttp.responseText;
}

let root = document.getElementById("root");
let currentHost = window.location.hostname;
let currentPort = window.location.port;
let events = httpGet(`http://${currentHost}:${currentPort}/score_page`);
const obj = JSON.parse(events);
const body = document.body,
tbl = document.createElement('table');
tbl.style.width = '100px';
tbl.style.border = '1px solid black';
tbl.cellPadding = "40px"

Object.keys(obj).forEach(function(key) {
    const tr = tbl.insertRow();
    const td = tr.insertCell();
    td.appendChild(document.createTextNode(key))
    const td1 = tr.insertCell();
    td1.appendChild(document.createTextNode(obj[key]))
})

setTimeout(() => {
document.location.reload();
}, 3000);

body.appendChild(tbl);
console.log("TEST")