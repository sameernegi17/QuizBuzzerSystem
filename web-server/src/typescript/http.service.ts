export class HttpService {

  public httpGet(url: string): string {
      const xmlHttp = new XMLHttpRequest();
      xmlHttp.open( "GET", url, false ); // false for synchronous request
      xmlHttp.send( );
      return xmlHttp.responseText;
  }

  public getHostBaseUrl(): string {
    const currentHost = window.location.hostname;
    const currentPort = window.location.port;
    return `http://${currentHost}:${currentPort}`;
  }

}