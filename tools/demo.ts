import { ApiPromise} from "@polkadot/api";
import { HttpProvider } from "@polkadot/rpc-provider";

import Web3 from "web3";
import {Account} from "web3-core";

async function main() {

	const web3Api = new Web3("http://127.0.0.1:9933");

	const substrateApi = await ApiPromise.create({
		provider: new HttpProvider("http://127.0.0.1:9933"),
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
	const aliceAddress = "0x6be02d1d3665660d22ff9624b7be0551ee1ac91b";

	let substrateAccount = await substrateApi.query.system.account(aliceAddress);
	console.log(`\nSubstrate: ${substrateAccount.data.free.toString()}`);

	let web3AccountBalance =  BigInt(await web3Api.eth.getBalance(aliceAddress));
	console.log(`Ethereum : ${web3AccountBalance}`);

	console.log(`\nTransferring 21 GLMR to random address`);
	await web3Api.eth.sendSignedTransaction(
		(await web3Api.eth.accounts.signTransaction(
			{
				value: `${21n * (10n**18n)}`,
				gasPrice: "0x01",
				gas: "0x21000",
				from: aliceAddress,
				to: "0x1111111111111111111111111111111111111111"
				// Alice private key
			}, "0x99B3C12287537E38C90A9219D4CB074A89A16E9CDB20BF85728EBD97C343E342"
		)).rawTransaction
	)

	substrateAccount = await substrateApi.query.system.account(aliceAddress);
	console.log(`\nSubstrate: ${substrateAccount.data.free.toString()}`);

	web3AccountBalance =  BigInt(await web3Api.eth.getBalance(aliceAddress));
	console.log(`Ethereum : ${web3AccountBalance}`);
}


main()
	.catch(console.error)
	.finally(() => process.exit());
