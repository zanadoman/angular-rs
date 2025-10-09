require('@dotenvx/dotenvx').config();

module.exports = {
  resources: [`http://127.0.0.1:${process.env['APP_PORT']}/api/docs/openapi.json`],
};
