/** @type {import('next').NextConfig} */
const nextConfig = {

  webpack: (config, { isServer }) => {
    const wasmExtensionRegExp = /\.wasm$/;
    if (!isServer) {
      config.resolve.fallback = {
        ...config.resolve.fallback,
        fs: false,
        net: false,
        tls: false,
      };
      config.resolve.extensions.push(".wasm");
      config.experiments = {
        asyncWebAssembly: true,
        syncWebAssembly: true,
        layers: true // for using `import { ... } from 'rust-nostr/nostr-sdk'` syntax
      };
    }
    config.resolve.extensions.push(".wasm");
    // config.module.rules.forEach((rule) => {
    //   (rule.oneOf || []).forEach((oneOf) => {
    //     if (oneOf.type === "asset/resource") {
    //       oneOf.exclude.push(wasmExtensionRegExp);
    //     }
    //   });
    // });
    // // Add wasm-loader
    // config.module.rules.push({
    //   test: /\.wasm$/,
    //   type: 'javascript/auto',
    //   use: {
    //     loader: 'wasm-loader',
    //   }
    // });
    config.experiments = {
      asyncWebAssembly: true,
      syncWebAssembly: true,
      layers: true // for using `import { ... } from 'rust-nostr/nostr-sdk'` syntax
    };
    return config;
  },
  eslint: {
    configure: (eslintConfig) => {
      eslintConfig.ignorePatterns = ["src/pkg/**/*"];
      return eslintConfig;
    },
  },
  async rewrites() {
    return [
      {
        source: '/pitchdeck',
        destination: '/pitchdeck/index.html',
      },
    ]
  },
};

export default nextConfig;
