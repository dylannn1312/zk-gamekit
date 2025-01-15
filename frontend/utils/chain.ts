import { GasPrice } from "@cosmjs/stargate";

export const gasPrice = GasPrice.fromString(process.env.GAS_PRICE);

export function shortAddress(address: string) {
    return `${address.slice(0, 6)}...${address.slice(-6)}`;
}
