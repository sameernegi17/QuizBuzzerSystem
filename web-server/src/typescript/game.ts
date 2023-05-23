import di from "./di.js";
import { QuestionService } from "./question.service.js";

interface ButtonEvent {

}

class GameComponent {

  private _socket: WebSocket;

  private resultElement: HTMLDivElement;

  constructor(
    private _questionService: QuestionService) {}
  
  public onInit() {
    this._socket = new WebSocket('ws://localhost:8000/websocket');

    this._socket.addEventListener('open', (event) => {
      console.log('WebSocket connection established.');
    });

    

    const generateButtonElement = document.getElementById('generate-button') as HTMLButtonElement;
    generateButtonElement.addEventListener('click', _ => this.onGenerateButtonClicked());

    const winnerDivElement = document.getElementById('winner') as HTMLDivElement;
    this.resultElement = winnerDivElement;

    this._socket.addEventListener('message', (event) => {
      this.resultElement.textContent = event.data;
    });
    
  }

  private sendMessage(message: string) {
    this._socket.send(message);
  }

  private onGenerateButtonClicked() {

    console.log('clicked')
    const newQuestion = this._questionService.getRandomQuestion();
    const questionElement = document.getElementById('question') as HTMLParagraphElement;

    questionElement.textContent = newQuestion;

    this._socket.send('reset');
  }



}

const gameComponent = new GameComponent(di.questionService);
gameComponent.onInit();
