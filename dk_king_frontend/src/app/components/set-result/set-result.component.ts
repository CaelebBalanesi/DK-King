import { Component, Input } from '@angular/core';
import { Set } from '../../interfaces/set';

@Component({
  selector: 'app-set-result',
  standalone: true,
  imports: [],
  templateUrl: './set-result.component.html',
  styleUrl: './set-result.component.scss'
})
export class SetResultComponent {

  @Input() set!: Set;
}