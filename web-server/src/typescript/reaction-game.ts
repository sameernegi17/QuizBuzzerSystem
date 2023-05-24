import di from "./di.js";
import { BackendService } from "./backend.service.js";

// interface Leaderboard {
//   entries: LeaderboardEntry[],
//   droppedOutButtonIndices: number[],
// }

// interface LeaderboardEntry {
//   buttonIndex: number,
//   reactionTimeMs: number,
// }

interface GameState {
  delay: number,
  droppedOutButtonIndices: number[],
  winners: [number, number][],
}


class ReactionGameComponent {

  private _startButtonElement: HTMLButtonElement;

  private _counterElement: HTMLDivElement;

  private _pressButtonSignalElement: HTMLDivElement;

  private _leaderboardTableElement = document.getElementById('leaderboard-table') as HTMLTableElement;

  private _droppedUsersListElement = document.getElementById('dropped-users-list') as HTMLUListElement;

  private _countdownInterval: number | undefined;

  constructor(private _backendService: BackendService) { };

  onInit() {
    this._startButtonElement = document.getElementById('start-btn') as HTMLButtonElement;
    this._counterElement = document.getElementById('counter') as HTMLDivElement;
    this._pressButtonSignalElement = document.getElementById('signal') as HTMLDivElement;
    this._leaderboardTableElement = document.getElementById('leaderboard-table') as HTMLTableElement;
    this._droppedUsersListElement = document.getElementById('dropped-users-list') as HTMLUListElement;

    this._startButtonElement.addEventListener('click', _ => this.onClickStartButton()); history

    this._countdownInterval = 0;

    const webSocket = new WebSocket('ws://localhost:9001/');

    webSocket.addEventListener('message', event => {
      const json = event.data;
      const leaderboard = JSON.parse(json) as GameState;
      return this.onGameStateUpdate(leaderboard);
    });

  }

  private onClickStartButton() {
    this.startGame();

  }

  private onCountdownEnds() {
    this._backendService.startReactionGame();
    this._countdownInterval = undefined;

    // TODO: remove - just for demo purposes

    // setTimeout(() => this.onReactionPhaseStarts(), 1500);

    // setTimeout(() => this.onGameStateUpdate({
    //   entries: [],
    //   droppedOutButtonIndices: [1],
    // }), 700);

    // setTimeout(() => this.onGameStateUpdate({
    //   entries: [],
    //   droppedOutButtonIndices: [1, 5],
    // }), 1300);

    // setTimeout(() => this.onGameStateUpdate({
    //   entries: [{ buttonIndex: 2, reactionTimeMs: 300 }],
    //   droppedOutButtonIndices: [1, 5],
    // }), 1800);

    // setTimeout(() => this.onGameStateUpdate({
    //   entries: [{ buttonIndex: 2, reactionTimeMs: 300 }, { buttonIndex: 0, reactionTimeMs: 420 }],
    //   droppedOutButtonIndices: [1, 5],
    // }), 1920);

    // setTimeout(() => this.onGameStateUpdate({
    //   entries: [{ buttonIndex: 2, reactionTimeMs: 300 }, { buttonIndex: 0, reactionTimeMs: 420 }, { buttonIndex: 3, reactionTimeMs: 1300 }],
    //   droppedOutButtonIndices: [1, 5],
    // }), 2800);
  }

  private onReactionPhaseStarts() {
    this._pressButtonSignalElement.classList.remove('d-none');
    this._countdownInterval = 0;
    this.endGame();
  }

  private onGameStateUpdate(gamestate: GameState) {
    console.log(gamestate);
    this.clearLeaderboard();

    // if countdown is not running, set timer to gameState.delay and start countdown
    if (this._countdownInterval === undefined) {
      this._countdownInterval = setTimeout(() => this.onReactionPhaseStarts(), gamestate.delay);
    }

    // Update the leaderboard table
    gamestate.winners
      .forEach(entry => {
        const row = document.createElement('tr');
        row.innerHTML = `
          <td>${entry[0]}</td>
          <td>${entry[1]}</td>
        `;
        this._leaderboardTableElement.tBodies[0].appendChild(row);
      });

    this.clearDroppedUsers();
    gamestate.droppedOutButtonIndices.forEach(buttonIndex => {
      const listItem = document.createElement('li');
      listItem.classList.add('list-group-item', 'd-flex', 'justify-content-between', 'align-items-center');
      listItem.innerHTML = `
        ${buttonIndex}
        <span class="badge bg-danger rounded-pill">x</span>
      `;
      this._droppedUsersListElement.appendChild(listItem);
    });

  }

  private startGame() {
    this._startButtonElement.disabled = true;
    this._pressButtonSignalElement.classList.add('d-none');
    this.setCountdown(3, () => this.onCountdownEnds());
    this.clearLeaderboard();
    this.clearDroppedUsers();
  }

  private endGame() {
    this._startButtonElement.disabled = false;
  }

  private setCountdown(count: number, callback: () => void) {

    this._counterElement.classList.remove('d-none'); // show the counter
    this._counterElement.innerText = "Get ready...";//count.toString();
    const countdownInterval = setInterval(() => {
      count--;
      this._counterElement.innerText = "Get really ready..."; //count.toString();
      if (count < 0) {
        clearInterval(countdownInterval);
        this._counterElement.classList.add('d-none'); // Hide the counter
        callback();
      }
    }, 1000);

  }

  private clearLeaderboard() {
    this._leaderboardTableElement.querySelector('tbody').innerHTML = '';
  }

  private clearDroppedUsers() {
    this._droppedUsersListElement.innerHTML = '';
  }

}

const reactionGameComponent = new ReactionGameComponent(di.backendService);
document.addEventListener('DOMContentLoaded', () => reactionGameComponent.onInit());
