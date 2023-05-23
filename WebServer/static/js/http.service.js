var HttpService = /** @class */ (function () {
    function HttpService() {
    }
    HttpService.prototype.httpGet = function (url) {
        var xmlHttp = new XMLHttpRequest();
        xmlHttp.open("GET", url, false); // false for synchronous request
        xmlHttp.send();
        return xmlHttp.responseText;
    };
    HttpService.prototype.getHostBaseUrl = function () {
        var currentHost = window.location.hostname;
        var currentPort = window.location.port;
        return "http://" + currentHost + ":" + currentPort;
    };
    return HttpService;
}());
export { HttpService };
