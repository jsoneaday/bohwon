export { default as WebIrys } from "@irys/sdk";
import { WebIrys } from "@irys/sdk";
import Solflare from "@solflare-wallet/sdk";

export async function getIrysInstance(
  url: string,
  token: string,
  wallet: {
    rpcUrl?: string | undefined;
    name?: string | undefined;
    provider: object;
  }
) {
  const provider = new Solflare();
  await provider.connect();

  const irys = new WebIrys({
    url,
    token,
    wallet: {
      rpcUrl: "https://solflare.com/ul/v1/connect",
      name: "solana",
      provider,
    },
  });

  return irys;
}

// wallet: {
//   rpcUrl: "https://solflare.com/ul/v1/connect",
//   name: "solana",
//   provider,
// },
