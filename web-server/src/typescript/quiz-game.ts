import { BackendService } from "./backend.service.js";
import di from "./di.js";
import { QuestionService } from "./question.service.js";

class QuizGameComponent {

  private _questionHeadingElement: HTMLHeadingElement;

  private _playerFirstHeadingElement: HTMLHeadingElement;

  private _generateQuestionButtonElement: HTMLButtonElement;

  constructor(
    private _questionService: QuestionService,
    private _backendService: BackendService) { };

  onInit() {

    this._questionHeadingElement = document.getElementById('question') as HTMLHeadingElement;
    this._playerFirstHeadingElement = document.getElementById('player-first') as HTMLHeadingElement;
    this._generateQuestionButtonElement = document.getElementById('generate-question-button') as HTMLButtonElement;

    this._generateQuestionButtonElement.addEventListener('click', () => this.onClickGenerateQuestionsButton());
  }

  private startGame() {
    this._playerFirstHeadingElement.textContent = '';

    this._generateQuestionButtonElement.disabled = true;
    const question = this._questionService.getRandomQuestion();
    this._questionHeadingElement.textContent = question;

    this._backendService.startQuizGame();

    // TODO: remove - only for demo purposes
    setTimeout(() => this.onFirstPlayerPressedButton(3), 2000);

  }

  private onClickGenerateQuestionsButton() {
    this.startGame();
  }

  // TODO: Connect with the backend and call this method
  private onFirstPlayerPressedButton(buttonIndex: number) {
    this._playerFirstHeadingElement.textContent = `Player » ${buttonIndex} « pressed the button first!`;
    this._generateQuestionButtonElement.disabled = false;
  }

}

const quizGameComponent = new QuizGameComponent(di.questionService, di.backendService);
quizGameComponent.onInit();


