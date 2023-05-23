import di from "./di.js";
import { HttpService } from "./http.service.js";

class ScorepageComponent {

  constructor(private _httpService: HttpService) {}

  public onInit() {
    const root = document.getElementById("root");
    const url = `${this._httpService.getHostBaseUrl()}/score_page`
    const events = this._httpService.httpGet(url);
    const obj = JSON.parse(events);
    const body = document.body,
    tbl = document.createElement('table');
    tbl.style.width = '100px';
    tbl.style.border = '1px solid black';
    tbl.cellPadding = "40px"

    Object.keys(obj).forEach(function(key) {
      const tr = tbl.insertRow();
      const td = tr.insertCell();
      td.appendChild(document.createTextNode(key))
      const td1 = tr.insertCell();
      td1.appendChild(document.createTextNode(obj[key]))
    })
  
    setTimeout(() => {
      document.location.reload();
    }, 3000);
    
    body.appendChild(tbl);
    console.log("TEST")
  }

}

const scorepageComponent = new ScorepageComponent(di.httpService);
scorepageComponent.onInit();

