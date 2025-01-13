'use client';
import { gasPrice } from "@/utils/chain";
import { calculateFee } from "@cosmjs/stargate";
import { useChain, useChains } from "@cosmos-kit/react";
import { Button, Typography } from "antd";
import Image from "next/image";

const { Text } = Typography;

export default function Header() {

    // useEffect(() => {
    //     async function handle() {
    //         if (address) {
    //             const client = await getSigningCosmWasmClient();
    //             console.log(address);
    //             client.sendTokens(
    //                 address,
    //                 "xion1d3fvv5eytvq0qgewvjszgfu3y92t8lah3r3lsa",
    //                 [{ denom: "uxion", amount: "5" }],
    //                 calculateFee(200000, gasPrice)
    //             );
    //         }
    //     }
    //     handle();
    // }, [address, getSigningCosmWasmClient])


    return (
        <header className="sticky top-0 z-50 w-full bg-secondary/50 backdrop-blur-md border-b border-gray-600 shadow-lg">
            <div className="flex gap-4 items-center h-9">
                <ConnectWallet />
            </div>
        </header>
    )
}

function ConnectWallet() {
    const chain = useChain(process.env.CHAIN_NAME);
    const { connect, openView, getSigningCosmWasmClient, address, isWalletConnected, logoUrl, username } = chain;
    return (
        isWalletConnected ?
            <>
                <Button onClick={openView} className='h-full flex items-center gap-2'>
                    <Image
                        alt={'Chain icon'}
                        src={logoUrl ?? ''}
                        width={28}
                        height={28}
                    />

                    <Text strong>{username}</Text>
                </Button>
            </>
            :
            <Button onClick={connect} type="primary">
                <Text strong>Connect Wallet</Text>
            </Button>
    )
}
