
// Simple default.
let config = {
  ENV: "PROD",
  PORT: 443,
  TLS: true,
}


// Try to replace any env variables if possible.
const hasEnv = import.meta.env.ENV && ['PROD', 'LOCAL', 'DEV'].find(import.meta.env.VITE_ENV);
if (hasEnv) {
  config = {
    ENV: import.meta.env.VITE_ENV || config.ENV,
    PORT: import.meta.env.VITE_PORT || config.PORT,
    TLS: import.meta.env.VITE_TLS || config.TLS,
  }
}


export default config;
