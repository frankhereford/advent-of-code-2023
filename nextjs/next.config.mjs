// Your existing imports
await import("./src/env.mjs");

/** @type {import("next").NextConfig} */
const config = {
  reactStrictMode: true,

  i18n: {
    locales: ["en"],
    defaultLocale: "en",
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
