/** @type {import('next').NextConfig} */
const nextConfig = {
    env: {
        DENOM: process.env.DENOM,
        CHAIN_NAME: process.env.CHAIN_NAME,
        GAS_PRICE: process.env.GAS_PRICE,
    },
    images: {
        remotePatterns: [
            {
                protocol: "https",
                hostname: "**",
            },
        ],
    },
    eslint: {
        ignoreDuringBuilds: true,
    },
}

export default nextConfig;
