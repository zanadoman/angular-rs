import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RootPage } from './root-page';
import { provideZonelessChangeDetection } from '@angular/core';
import { provideHttpClient } from '@angular/common/http';
import { provideHttpClientTesting } from '@angular/common/http/testing';

describe('RootPage', () => {
  let component: RootPage;
  let fixture: ComponentFixture<RootPage>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [RootPage],
      providers: [
        provideZonelessChangeDetection(),
        provideHttpClient(),
        provideHttpClientTesting(),
      ],
    }).compileComponents();

    fixture = TestBed.createComponent(RootPage);
    component = fixture.componentInstance;
    await fixture.whenStable();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
