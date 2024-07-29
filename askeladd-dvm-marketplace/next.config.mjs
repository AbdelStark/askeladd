/** @type {import('next').NextConfig} */
const nextConfig = {
  webpack: (config, { isServer }) => {
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
    config.experiments = {
      asyncWebAssembly: true,
      syncWebAssembly: true,
      layers: true // for using `import { ... } from 'rust-nostr/nostr-sdk'` syntax
    };
    return config;
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
