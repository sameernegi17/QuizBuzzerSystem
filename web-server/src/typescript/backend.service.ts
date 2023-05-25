import { HttpService } from "./http.service.js";

export class BackendService {

  constructor(private _httpService: HttpService) { };

  public getHostBaseUrl() {
    return `${this._httpService.getHostBaseUrl()}/reaction-game/start`;
  }

  public startReactionGame() {
    console.log("about to start reaction game");
    // this._httpService.httpGet(`${this._httpService.getHostBaseUrl()}/reaction-game/start`);
    console.log(`${this._httpService.getHostBaseUrl()}/reaction-game/start`);
    fetch(`${this._httpService.getHostBaseUrl()}/reaction-game/start`, {
      method: 'GET',
    }).then(response => {
      console.log(JSON.stringify(response));
    });
  }

  public startQuizGame() {
    this._httpService.httpGet(`${this._httpService.getHostBaseUrl()}/quiz-game/start`);
  }

}