'use client';
import { gasPrice } from "@/utils/chain";
import { calculateFee } from "@cosmjs/stargate";
import { useChain, useChains } from "@cosmos-kit/react";
import { Button, Menu, MenuProps, Typography } from "antd";
import Image from "next/image";
import Link from "next/link";
import { useState } from "react";

const { Text } = Typography;

type MenuItem = Required<MenuProps>['items'][number];

const menuItems: MenuItem[] = [
  {
    label: <Link href={'/'} className='hover:!text-muted font-bold text-base'>GAMES</Link>,
    key: 'top-games',

  },
  {
    label: <Link href={'/'} className='hover:!text-muted font-bold text-base'>WINNERS</Link>,
    key: 'winners',
  }
];

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
    const [currentPage, setCurrentPage] = useState('');

    const onClick: MenuProps['onClick'] = (e) => {
      setCurrentPage(e.key);
    };

    return (
        <header className="sticky top-0 z-50 w-full bg-secondary/50 backdrop-blur-md border-b border-gray-400">
            <div className="flex gap-8 items-center h-20 py-5 w-full">
                <Link href='/' className="flex-1"><Text className='text-4xl pr-8 text-primary font-extrabold'>ZKGameboard</Text></Link>

                <Menu onClick={onClick} selectedKeys={[currentPage]} mode="horizontal" items={menuItems} className='border-b-transparent' />
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
            <Button onClick={connect} type="primary" className="h-full">
                <Text strong>Connect Wallet</Text>
            </Button>
    )
}
