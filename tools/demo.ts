//@ts-nocheck

import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { HttpProvider } from "@polkadot/rpc-provider";

import Web3 from "web3";
import {Account} from "web3-core";

async function main() {

	const web3Api = new Web3("http://127.0.0.1:9933");

	const substrateApi = await ApiPromise.create({
		provider: new WsProvider("ws://127.0.0.1:9944"),
		types: {
			Balance: "u128",
			AccountId: "H160",
			Address: "AccountId",
			LookupSource: "AccountId",
			Account: {
				nonce: "U256",
				balance: "U256",
			}
		},
	});
	const keyring = new Keyring({ type: 'ethereum' });
	const alicePair = keyring.addFromUri('0x99B3C12287537E38C90A9219D4CB074A89A16E9CDB20BF85728EBD97C343E342', { name: 'gerald'}, 'ethereum');

	// Genesis account: 123456_123_000_000_000_000_000 tokens  (18 decimals)
	const aliceAddress = "0x6be02d1d3665660d22ff9624b7be0551ee1ac91b";

	let substrateAccount = await substrateApi.query.system.account(aliceAddress);
	console.log(`\nAlice balance on Substrate: ${BigInt(substrateAccount.data.free.toString()) / (10n**18n)}`);

	let web3AccountBalance =  BigInt(await web3Api.eth.getBalance(aliceAddress));
	console.log(`Alice balance on Ethereum : ${web3AccountBalance / (10n**18n)}`);

	const glmrCount = 456n * (10n**18n);
	console.log(`\nTransferring 456 GLMR to 0x1..1 address (using Eth web3 SDK)`);

	let start = Date.now();
	await web3Api.eth.sendSignedTransaction(
		(await web3Api.eth.accounts.signTransaction(
			{
				value: `${glmrCount}`,
				gasPrice: "0x01",
				gas: "0x21000",
				from: aliceAddress,
				to: "0x1111111111111111111111111111111111111111"
				// Alice private key
			}, "0x99B3C12287537E38C90A9219D4CB074A89A16E9CDB20BF85728EBD97C343E342"
		)).rawTransaction
	)
	console.log(`Transfer took: ${Date.now() - start}`);
	substrateAccount = await substrateApi.query.system.account(aliceAddress);
	console.log(`\nAlice balance on Substrate: ${BigInt(substrateAccount.data.free.toString()) / (10n**18n)}`);

	web3AccountBalance =  BigInt(await web3Api.eth.getBalance(aliceAddress));
	console.log(`Alice balance on Ethereum : ${web3AccountBalance / (10n**18n)}`);


	substrateAccount = await substrateApi.query.system.account("0x1111111111111111111111111111111111111111");
	console.log(`\n0x1..1 balance (Substrate): ${BigInt(substrateAccount.data.free.toString()) / (10n**18n)}`);

	web3AccountBalance =  BigInt(await web3Api.eth.getBalance("0x1111111111111111111111111111111111111111"));
	console.log(`0x1..1 balance (Ethereum) : ${web3AccountBalance / (10n**18n)}`);


	console.log(`\nTransferring 456 GLMR to 0x1..1 address (using Substrate SDK)`);

	const nonce = await substrateApi.rpc.system.accountNextIndex(alicePair.address);
	start = Date.now();
	await new Promise<{ block: string, address: string }>(async (resolve, reject) => {
		const unsub = await substrateApi.tx.balances
		.transfer("0x1111111111111111111111111111111111111111", glmrCount)
		.signAndSend(alicePair, { nonce }, (result) => {
				if (result.status.isInBlock) {
					console.log(`Payment included at blockHash ${result.status.asInBlock} (waiting finalization...)`);
					resolve();
				} else if (result.status.isFinalized) {
					console.log(`Payment finalized at blockHash ${result.status.asFinalized}`);
					unsub();
				}
			});
	});
	console.log(`Transfer took: ${Date.now() - start}`);

	substrateAccount = await substrateApi.query.system.account(aliceAddress);
	console.log(`\nAlice balance on Substrate: ${BigInt(substrateAccount.data.free.toString()) / (10n**18n)}`);

	web3AccountBalance =  BigInt(await web3Api.eth.getBalance(aliceAddress));
	console.log(`Alice balance on Ethereum : ${web3AccountBalance / (10n**18n)}`);


	substrateAccount = await substrateApi.query.system.account("0x1111111111111111111111111111111111111111");
	console.log(`\n0x1..1 balance (Substrate): ${BigInt(substrateAccount.data.free.toString()) / (10n**18n)}`);

	web3AccountBalance =  BigInt(await web3Api.eth.getBalance("0x1111111111111111111111111111111111111111"));
	console.log(`0x1..1 balance (Ethereum) : ${web3AccountBalance / (10n**18n)}`);
}


main()
	.catch(console.error)
	.finally(() => process.exit());
