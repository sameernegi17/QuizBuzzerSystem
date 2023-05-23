import di from './di.js';
import { HttpService } from './http.service.js'

class CounterComponent {

  constructor(private _httpService: HttpService) {}

  public onInit() {

    const root = document.getElementById("root");
    const url = `${this._httpService.getHostBaseUrl()}/add`;
    let events = this._httpService.httpGet(url);
    let data = document.createElement("p");
    data.innerText = events;
    root.appendChild(data);

  }

}

const counterComponent = new CounterComponent(di.httpService);
counterComponent.onInit();

