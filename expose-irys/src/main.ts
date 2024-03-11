export { default as WebIrys } from "@irys/sdk";
import { WebIrys } from "@irys/sdk";

export { default as Solflare } from "@solflare-wallet/sdk";
import Solflare from "@solflare-wallet/sdk";

export async function getSolflareInstance(immediateConnect: boolean) {
  const provider = new Solflare();
  if (immediateConnect) provider.connect();
  return provider;
}

export async function getIrysInstance(
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

// wallet: {
//   rpcUrl: "https://solflare.com/ul/v1/connect",
//   name: "solana",
//   provider,
// },
