import di from "./di.js";
var ScorepageComponent = /** @class */ (function () {
    function ScorepageComponent(_httpService) {
        this._httpService = _httpService;
    }
    ScorepageComponent.prototype.onInit = function () {
        var root = document.getElementById("root");
        var url = this._httpService.getHostBaseUrl() + "/score_page";
        var events = this._httpService.httpGet(url);
        var obj = JSON.parse(events);
        var body = document.body, tbl = document.createElement('table');
        tbl.style.width = '100px';
        tbl.style.border = '1px solid black';
        tbl.cellPadding = "40px";
        Object.keys(obj).forEach(function (key) {
            var tr = tbl.insertRow();
            var td = tr.insertCell();
            td.appendChild(document.createTextNode(key));
            var td1 = tr.insertCell();
            td1.appendChild(document.createTextNode(obj[key]));
        });
        setTimeout(function () {
            document.location.reload();
        }, 3000);
        body.appendChild(tbl);
        console.log("TEST");
    };
    return ScorepageComponent;
}());
var scorepageComponent = new ScorepageComponent(di.httpService);
scorepageComponent.onInit();
