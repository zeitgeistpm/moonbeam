import { expect } from "chai";
import Keyring from "@polkadot/keyring";

import { describeDevMoonbeam } from "../util/setup-dev-tests";

import { GENESIS_ACCOUNT, ETHAN, RANDOM_PRIV_KEY } from "../util/constants";
import { createContract, createTransfer } from "../util/transactions";


describeDevMoonbeam("Poor account shoult be able to estimate a call with gas price", (context) => {
  it("Poor account shoult be able to estimate a call with gas price", async function () {
    const keyring = new Keyring({ type: "ethereum" });
    const randomAccount = await keyring.addFromUri(RANDOM_PRIV_KEY, null, "ethereum");

    console.log(BigInt(await context.web3.eth.getBalance(randomAccount.address, 0)));
  
    const { contract, rawTx } = await createContract(context.web3, "Incrementer");
    await context.createBlock({ transactions: [rawTx] });

    // Transfer 50 000 Gwei to random account
    await context.createBlock({
      transactions: [await createTransfer(context.web3, randomAccount.address, 50_000n * 1_000_000_000n)],
    });
    expect(await context.web3.eth.getBalance(randomAccount.address, 2)).to.equal(
      (50_000n * 1_000_000_000n).toString()
    );
    console.log(BigInt(await context.web3.eth.getBalance(randomAccount.address, 2)));

    // Estimate Gas of Approve with Poor Account and no Gas Limit Set
    expect(
      await contract.methods.sum(42).estimateGas({
        from: randomAccount.address,
        gasPrice: 1_000_000_000n,
      })
    ).to.lessThanOrEqual(50_000);
  });
});
