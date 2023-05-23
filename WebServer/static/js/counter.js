import di from './di.js';
var CounterComponent = /** @class */ (function () {
    function CounterComponent(_httpService) {
        this._httpService = _httpService;
    }
    CounterComponent.prototype.onInit = function () {
        var root = document.getElementById("root");
        var url = this._httpService.getHostBaseUrl() + "/add";
        var events = this._httpService.httpGet(url);
        var data = document.createElement("p");
        data.innerText = events;
        root.appendChild(data);
    };
    return CounterComponent;
}());
var counterComponent = new CounterComponent(di.httpService);
counterComponent.onInit();
