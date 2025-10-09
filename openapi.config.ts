import { GeneratorConfig } from 'ng-openapi';
import '@dotenvx/dotenvx/config';

const config: GeneratorConfig = {
  input: `http://127.0.0.1:${process.env['APP_PORT']}/api/docs/openapi.json`,
  output: 'src/api',
  options: {
    dateType: 'string',
    enumStyle: 'enum',
  },
};

export default config;
