// Your existing imports
await import("./src/env.mjs");
import packageJson from './package.json' assert { type: 'json' }

let defaultDay = packageJson.defaultDay;

/** @type {import("next").NextConfig} */
const config = {
  reactStrictMode: true,

  i18n: {
    locales: ["en"],
    defaultLocale: "en",
  },

  async redirects () {
    return [
      {
        source: '/',
        destination: `/${defaultDay}`,
        permanent: false
      }
    ]
  },

  // Adding webpack configuration
  webpack: (config) => {
    // Enable WebAssembly
    config.experiments = {
      asyncWebAssembly: true,
      layers: true, 
      // or if you prefer syncWebAssembly: true (deprecated)
    };

    return config;
  },
};

export default config;
