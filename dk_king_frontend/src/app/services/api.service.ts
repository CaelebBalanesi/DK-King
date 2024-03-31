import { Injectable } from '@angular/core';
import { HttpClient } from  '@angular/common/http';
import { environment } from '../enviroments/enviroment';

@Injectable({
  providedIn: 'root',
})
export class ApiService {

  constructor(private http: HttpClient) { }

  getSets() {
    return this.http.get( environment.api_url + '/sets');
  }

  sendSet(score1: number, score2: number, winnerName: string) {
    const body = { score1, score2, winner_name: winnerName };
    return this.http.post( environment.api_url + '/add_set', body);
  }
}
