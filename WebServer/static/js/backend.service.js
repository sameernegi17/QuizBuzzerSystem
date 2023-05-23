var BackendService = /** @class */ (function () {
    function BackendService(_httpService) {
        this._httpService = _httpService;
    }
    ;
    BackendService.prototype.resetGame = function () {
        this._httpService.httpGet(this._httpService.getHostBaseUrl() + "/reset-game");
    };
    return BackendService;
}());
export { BackendService };
