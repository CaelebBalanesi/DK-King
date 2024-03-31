import { Component } from '@angular/core';
import { ApiService } from '../../services/api.service';

@Component({
  selector: 'app-set-adder',
  standalone: true,
  imports: [],
  templateUrl: './set-adder.component.html',
  styleUrl: './set-adder.component.scss'
})
export class SetAdderComponent {
  winner_selected: string = "";

  constructor(private api: ApiService) { }

  select_winner(username: string) {
    this.winner_selected = username;
  }

  select_scoreline(score: number) {
    if (this.winner_selected == "cal") {
      console.log("cal");
      this.api.sendSet(score, 2, "cal").subscribe((response: Object) => {
        console.log(response);
      });
    } else if (this.winner_selected == "jen") {
      this.api.sendSet(2, score, "jen").subscribe((response: Object) => {
        console.log(response);
      });
    }
    this.winner_selected = "";
  }
}
