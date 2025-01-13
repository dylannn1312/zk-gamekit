'use client';

import { ChainProvider } from "@cosmos-kit/react";
import { assets, chains } from "chain-registry";
import { wallets as keplrWallet } from "@cosmos-kit/keplr";

export default function CosmosChainProvider({
    children,
}: {
    children: React.ReactNode;
}) {

    return (
        <ChainProvider chains={chains} wallets={[...keplrWallet]} assetLists={assets}>
            {children}
        </ChainProvider>
    )
}
