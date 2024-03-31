import { Component } from '@angular/core';
import { SetAdderComponent } from '../components/set-adder/set-adder.component';
import { SetResultComponent } from '../components/set-result/set-result.component';
import { SetTallyComponent } from '../components/set-tally/set-tally.component';
import { Set } from '../interfaces/set';
import { ApiService } from '../services/api.service';

@Component({
  selector: 'app-home-page',
  standalone: true,
  imports: [
    SetAdderComponent,
    SetResultComponent,
    SetTallyComponent
  ],
  templateUrl: './home-page.component.html',
  styleUrl: './home-page.component.scss'
})

export class HomePageComponent {

  constructor(private apiService: ApiService) { }

  ngOnInit() {
    this.apiService.getSets().subscribe((sets: Object) => {
      console.log(sets);
      this.sets = sets as Set[];
    });
  }

  sets: Set[] = [];
}
