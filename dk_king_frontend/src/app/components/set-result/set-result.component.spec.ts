import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SetResultComponent } from './set-result.component';

describe('SetResultComponent', () => {
  let component: SetResultComponent;
  let fixture: ComponentFixture<SetResultComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SetResultComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(SetResultComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
