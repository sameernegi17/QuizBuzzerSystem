import { BackendService } from "./backend.service";
import di from "./di.js";
import { QuestionService } from "./question.service.js";

interface ButtonEvents {
  // buttonEvents:  
}

interface ButtonEvent {
  buttonIndex: number,
}

class GameComponent {

  private _socket: WebSocket;

  // private _winnerListElement: HTMLDivElement;

  constructor(
    private _questionService: QuestionService,
    private _backendService: BackendService) {}
  
  public onInit() {
    this._socket = new WebSocket('ws://localhost:8000/websocket');

    this._socket.addEventListener('open', (event) => {
      console.log('WebSocket connection established.');
    });

    

    const generateButtonElement = document.getElementById('generate-button') as HTMLButtonElement;
    generateButtonElement.addEventListener('click', _ => this.onGenerateButtonClicked());

    // this._winnterListElement = document.getElementById('winner') as HTMLDivElement;
    
    this._socket.addEventListener('message', (event) => {

      console.log(`Message received: ${event.data}`)

      // const message = event.data as string;
      // const ids = message.split(",").map(num => +num);

      // ids.forEach(id => {

      //   const element = document.getElementById(`winner-${id}`);
      //   if(element === null) {
      //     const child = document.createElement('li') as HTMLLIElement;
      //     child.id = `winner-${id}`;
      //     child.textContent = `Button ${id} pressed!`;
      //     this._winnerListElement.
      //   }

      // }

      // this.resultElement.textContent = event.data;
    });
    
  }

  private onGenerateButtonClicked() {

    const newQuestion = this._questionService.getRandomQuestion();
    const questionElement = document.getElementById('question') as HTMLParagraphElement;
    questionElement.textContent = newQuestion;

    this._backendService.resetGame();
  }



}

const gameComponent = new GameComponent(di.questionService, di.backendService);
gameComponent.onInit();
