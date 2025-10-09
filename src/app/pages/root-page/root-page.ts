import { Component, inject } from '@angular/core';
import { FormControl, FormGroup, ReactiveFormsModule } from '@angular/forms';
import { AuthenticationService, Credentials, User } from '../../../api';
import { firstValueFrom } from 'rxjs';
import { HttpErrorResponse, HttpResponse } from '@angular/common/http';

@Component({
  selector: 'app-root-page',
  imports: [ReactiveFormsModule],
  templateUrl: './root-page.ng.html',
  styleUrl: './root-page.scss',
})
export class RootPage {
  private readonly _authenticationService = inject(AuthenticationService);

  protected readonly registerForm = new FormGroup({
    username: new FormControl(''),
    password: new FormControl(''),
  });

  protected readonly loginForm = new FormGroup({
    username: new FormControl(''),
    password: new FormControl(''),
  });

  protected async onRegister(): Promise<void> {
    try {
      RootPage.showValue(
        await firstValueFrom(this._authenticationService.register(this.registerForm.value as User)),
      );
    } catch (err) {
      RootPage.showError(err);
    }
  }

  protected async onLogin(): Promise<void> {
    try {
      RootPage.showValue(
        await firstValueFrom(
          this._authenticationService.login(this.loginForm.value as Credentials),
        ),
      );
    } catch (err) {
      RootPage.showError(err);
    }
  }

  protected async onLogout(): Promise<void> {
    try {
      RootPage.showResponse(await firstValueFrom(this._authenticationService.logout('response')));
    } catch (err) {
      RootPage.showError(err);
    }
  }

  private static showValue<T>(val: T): void {
    window.alert(JSON.stringify(val, null, 2));
  }

  private static showError(err: unknown): void {
    if (err instanceof HttpErrorResponse) {
      RootPage.showValue(err.error ?? err.status);
    }
    throw err;
  }

  private static showResponse(res: HttpResponse<object>): void {
    RootPage.showValue(res.body ?? res.status);
  }
}
