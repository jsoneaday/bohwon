export { default as WebIrys } from "@irys/sdk";
import { WebIrys } from "@irys/sdk";

export { default as Solflare } from "@solflare-wallet/sdk";
import Solflare from "@solflare-wallet/sdk";

export function getSolflareInstance(immediateConnect: boolean) {
  const provider = new Solflare();
  if (immediateConnect) provider.connect();
  return provider;
}

export function getIrysInstance(
  url: string,
  token: string,
  wallet: {
    rpcUrl?: string | undefined;
    name?: string | undefined;
    provider: object;
  }
) {
  const irys = new WebIrys({
    url,
    token,
    wallet,
  });

  return irys;
}

export function toAtomic(webIrys: WebIrys, value: number) {
  return webIrys.utils.toAtomic(value);
}

export function fromAtomic(webIrys: WebIrys, value: number) {
  return webIrys.utils.fromAtomic(value);
}

export function token(webIrys: WebIrys) {
  return webIrys.token;
}

// wallet: {
//   rpcUrl: "https://solflare.com/ul/v1/connect",
//   name: "solana",
//   provider,
// },
