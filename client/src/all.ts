import { DataSource, PoRClient, loadConfigFromFile } from "@primuslabs/por-client-sdk";

const interval = 1800;
console.log(`The interval: ${interval} s.`)

async function main() {
  try {
    const config = loadConfigFromFile();
    const ds = new DataSource.ExchangeManager(config.exchanges);

    const params = {
      binanceSpot: () => ds.binance?.getSpotAccountInfoRequests(),
      binanceUsdSFuture: () => ds.binance?.getUsdSFutureAccountBalanceV3Requests(),
      binanceUnified: () => ds.binance?.getUnifiedAccountBalanceRequests(),
      asterSpot: () => ds.aster?.getSpotAccountRequests(),
      asterUsdSFuture: () => ds.aster?.getUsdSFutureBalanceRequests(),
    };

    const client = new PoRClient(config.app,);
    const result = await client.run(params);
    // console.log("result", JSON.stringify(result));
    console.log('proof fixture(json):', JSON.parse(result?.proof_fixture ?? "{}"));
  } catch (err: any) {
    console.log("err:", err?.message, JSON.stringify(err));
  }

  console.log(`⏳ Next in ${interval} s...`);
}

main();
setInterval(main, interval * 1000);

