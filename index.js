import init, { fetch_ticker } from './pkg/rust_bitcoin_lib.js';

init().then(async () => {
  const price = await fetch_ticker();
  document.getElementById("bitcoinPriceId").innerHTML = new Intl.NumberFormat("de-DE", { style: "currency", currency: "EUR" }).format(
    price
  );
})
