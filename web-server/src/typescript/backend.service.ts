import { HttpService } from "./http.service.js";

export class BackendService {

  constructor(private _httpService: HttpService) {};

  public resetGame() {
    this._httpService.httpGet(`${this._httpService.getHostBaseUrl()}/reset-game`);
  }

}