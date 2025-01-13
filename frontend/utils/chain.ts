import { GasPrice } from "@cosmjs/stargate";

export const gasPrice = GasPrice.fromString(process.env.GAS_PRICE);
