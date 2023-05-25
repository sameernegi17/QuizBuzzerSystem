import { BackendService } from "./backend.service.js";
import di from "./di.js";

class SoundGameComponent {

  private _startGameButtonElement: HTMLButtonElement;

  private _reactionGameStarted = false;

  constructor(private _backendService: BackendService) { };

  onInit() {
    this._startGameButtonElement = document.getElementById('start-game-button') as HTMLButtonElement;
    this._startGameButtonElement.addEventListener('click', _ => this.onClickStartButton());
  }

  private onClickStartButton() {
    if (!this._reactionGameStarted) {
      this._backendService.startSoundGame();
      this._reactionGameStarted = true;
      this._startGameButtonElement.disabled = true;
    }
  }

}

const soundGameComponent = new SoundGameComponent(di.backendService);
soundGameComponent.onInit();