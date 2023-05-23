import di from "./di.js";
var GameComponent = /** @class */ (function () {
    function GameComponent(_questionService) {
        this._questionService = _questionService;
    }
    GameComponent.prototype.onInit = function () {
        var _this = this;
        this._socket = new WebSocket('ws://localhost:8000/websocket');
        this._socket.addEventListener('open', function (event) {
            console.log('WebSocket connection established.');
        });
        var generateButtonElement = document.getElementById('generate-button');
        generateButtonElement.addEventListener('click', function (_) { return _this.onGenerateButtonClicked(); });
        var winnerDivElement = document.getElementById('winner');
        this.resultElement = winnerDivElement;
        this._socket.addEventListener('message', function (event) {
            _this.resultElement.textContent = event.data;
        });
    };
    GameComponent.prototype.sendMessage = function (message) {
        this._socket.send(message);
    };
    GameComponent.prototype.onGenerateButtonClicked = function () {
        console.log('clicked');
        var newQuestion = this._questionService.getRandomQuestion();
        var questionElement = document.getElementById('question');
        questionElement.textContent = newQuestion;
        this._socket.send('reset');
    };
    return GameComponent;
}());
var gameComponent = new GameComponent(di.questionService);
gameComponent.onInit();
