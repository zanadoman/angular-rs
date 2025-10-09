import { Component, inject } from '@angular/core';
import { FormControl, FormGroup, ReactiveFormsModule } from '@angular/forms';
import { HttpClient } from '@angular/common/http';
import { API } from '../../app.config';

@Component({
  selector: 'app-root',
  imports: [ReactiveFormsModule],
  templateUrl: './root.html',
  styleUrl: './root.scss'
})
export class Root {
  private readonly _httpClient = inject(HttpClient);

  protected readonly registerForm = new FormGroup({
    name: new FormControl(''),
    password: new FormControl('')
  });
  protected readonly loginForm = new FormGroup({
    name: new FormControl(''),
    password: new FormControl('')
  });

  protected onRegister(): void {
    this._httpClient.post(`${API}/register`, this.registerForm.value).subscribe({
      error: err => window.alert(`Register error: ${JSON.stringify(err.error)}`),
      complete: () => window.alert('Register completed')
    });
  }

  protected onLogin(): void {
    this._httpClient.post(`${API}/login`, this.loginForm.value, {
      withCredentials: true
    }).subscribe({
      error: err => window.alert(`Login error: ${JSON.stringify(err.error)}`),
      complete: () => window.alert('Login completed')
    });
  }

  protected onLogout(): void {
    this._httpClient.post(`${API}/logout`, this.loginForm.value, {
      withCredentials: true
    }).subscribe({
      error: err => window.alert(`Logout error: ${JSON.stringify(err.error)}`),
      complete: () => window.alert('Logout completed')
    });
  }
}
