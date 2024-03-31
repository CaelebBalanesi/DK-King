import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SetAdderComponent } from './set-adder.component';

describe('SetAdderComponent', () => {
  let component: SetAdderComponent;
  let fixture: ComponentFixture<SetAdderComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SetAdderComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(SetAdderComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
