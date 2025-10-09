import type { CapacitorConfig } from '@capacitor/cli';
import '@dotenvx/dotenvx/config';

const config: CapacitorConfig = {
  appId: 'com.example.angular_rs',
  appName: 'angular-rs',
  webDir: 'dist/angular-rs/browser',
  server: {
    url: process.env.CAPACITOR_URL,
    cleartext: true,
  },
};

export default config;
