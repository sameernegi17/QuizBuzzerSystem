import di from "./di.js";
import { BackendService } from "./backend.service.js";

interface Leaderboard {
  entries: LeaderboardEntry[],
  droppedOutButtonIndices: number[],
}

interface LeaderboardEntry {
  buttonIndex: number,
  reactionTimeMs: number,
}


class ReactionGameComponent {

  private _startButtonElement: HTMLButtonElement;

  private _counterElement: HTMLDivElement;

  private _pressButtonSignalElement: HTMLDivElement;


  constructor(private _backendService: BackendService) { };

  onInit() {
    this._startButtonElement = document.getElementById('start-btn') as HTMLButtonElement;
    this._counterElement = document.getElementById('counter') as HTMLDivElement;
    this._pressButtonSignalElement = document.getElementById('signal') as HTMLDivElement;

    this._startButtonElement.addEventListener('click', _ => this.onClickStartButton());

    const actionSocket = new WebSocket('ws://localhost:8000/reaction-game/action');
    const leaderboardSocket = new WebSocket('ws://localhost:8000/reaction-game/leaderboard');

    actionSocket.addEventListener('message', _ => this.onReactionPhaseStarts());
    leaderboardSocket.addEventListener('message', event => {
      const json = event.data;
      const leaderboard = JSON.parse(json) as Leaderboard;
      return this.onLeaderboardChanges(leaderboard);
    });

  }

  private onClickStartButton() {
    this.startGame();
  }

  private onCountdownEnds() {
    this._backendService.startReactionGame();
  }

  private onReactionPhaseStarts() {
    this._pressButtonSignalElement.innerText = 'React now!';
    this._pressButtonSignalElement.classList.add('bg-danger', 'text-white');
    this.endGame();
  }

  private onLeaderboardChanges(leaderboard: Leaderboard) {
    console.log(leaderboard);
  }



  private startGame() {
    this._startButtonElement.disabled = true;
    this._pressButtonSignalElement.classList.add('d-none');
    this.setCountdown(3, () => this.onCountdownEnds());
  }

  private endGame() {
    this._startButtonElement.disabled = false;
  }

  private setCountdown(count: number, callback: () => void) {

    this._counterElement.classList.remove('d-none'); // show the counter
    this._counterElement.innerText = count.toString();
    const countdownInterval = setInterval(() => {
      count--;
      this._counterElement.innerText = count.toString();
      if (count < 0) {
        clearInterval(countdownInterval);
        this._counterElement.classList.add('d-none'); // Hide the counter
        callback();
      }
    }, 1000);

  }

}

const reactionGameComponent = new ReactionGameComponent(di.backendService);
document.addEventListener('DOMContentLoaded', () => reactionGameComponent.onInit());
