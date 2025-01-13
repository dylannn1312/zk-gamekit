declare global {
  namespace NodeJS {
    interface ProcessEnv {
        DENOM: string;
        CHAIN_NAME: string;
        GAS_PRICE: string;
    }
  }
}
export {}
