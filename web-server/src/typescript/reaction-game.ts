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

  private _leaderboardTableElement = document.getElementById('leaderboard-table') as HTMLTableElement;

  private _droppedUsersListElement = document.getElementById('dropped-users-list') as HTMLUListElement;


  constructor(private _backendService: BackendService) { };

  onInit() {
    this._startButtonElement = document.getElementById('start-btn') as HTMLButtonElement;
    this._counterElement = document.getElementById('counter') as HTMLDivElement;
    this._pressButtonSignalElement = document.getElementById('signal') as HTMLDivElement;

    this._startButtonElement.addEventListener('click', _ => this.onClickStartButton());

    const actionSocket = new WebSocket('ws://localhost:8000/reaction-game/action');
    const leaderboardSocket = new WebSocket('ws://localhost:8000/reaction-game/leaderboard');

    setTimeout(() => this.onLeaderboardChanges({
      entries: [{ buttonIndex: 2, reactionTimeMs: 2300 }, { buttonIndex: 0, reactionTimeMs: 2420 }, { buttonIndex: 3, reactionTimeMs: 2767 }],
      droppedOutButtonIndices: [1, 5],
    }), 3000);

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

    this.clearLeaderboard();

    // Update the leaderboard table
    leaderboard.entries
      .forEach(entry => {
        const row = document.createElement('tr');
        row.innerHTML = `
          <td>${entry.buttonIndex}</td>
          <td>${entry.reactionTimeMs}</td>
        `;
        this._leaderboardTableElement.tBodies[0].appendChild(row);
        console.log("Added")
        console.log(entry);
      });

    this.clearDroppedUsers();

    leaderboard.droppedOutButtonIndices.forEach(buttonIndex => {
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

  private clearLeaderboard() {
    this._leaderboardTableElement.querySelector('tbody').innerHTML = '';
  }

  private clearDroppedUsers() {
    this._droppedUsersListElement.innerHTML = '';
  }

}

const reactionGameComponent = new ReactionGameComponent(di.backendService);
document.addEventListener('DOMContentLoaded', () => reactionGameComponent.onInit());
