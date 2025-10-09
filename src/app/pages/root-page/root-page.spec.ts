import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RootPage } from './root-page';

describe('RootPage', () => {
  let component: RootPage;
  let fixture: ComponentFixture<RootPage>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [RootPage],
    }).compileComponents();

    fixture = TestBed.createComponent(RootPage);
    component = fixture.componentInstance;
    await fixture.whenStable();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
