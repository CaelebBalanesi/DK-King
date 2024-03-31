import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SetTallyComponent } from './set-tally.component';

describe('SetTallyComponent', () => {
  let component: SetTallyComponent;
  let fixture: ComponentFixture<SetTallyComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SetTallyComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(SetTallyComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
