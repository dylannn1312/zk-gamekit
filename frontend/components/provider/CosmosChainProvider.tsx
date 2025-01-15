'use client';

import { ChainProvider } from "@cosmos-kit/react";
import { assets, chains } from "chain-registry";
import { wallets as keplrWallet } from "@cosmos-kit/keplr";
import { wallets as leapWallet } from "@cosmos-kit/leap";

export default function CosmosChainProvider({
    children,
}: {
    children: React.ReactNode;
}) {

    return (
        <ChainProvider chains={chains} wallets={[...keplrWallet, ...leapWallet]} assetLists={assets}>
            {children}
        </ChainProvider>
    )
}
