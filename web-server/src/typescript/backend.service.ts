import { HttpService } from "./http.service.js";

export class BackendService {

  constructor(private _httpService: HttpService) { };

  public startReactionGame() {
    this._httpService.httpGet(`${this._httpService.getHostBaseUrl()}/reaction-game/start`);
  }

  public startQuizGame() {
    this._httpService.httpGet(`${this._httpService.getHostBaseUrl()}/quiz-game/start`);
  }

}