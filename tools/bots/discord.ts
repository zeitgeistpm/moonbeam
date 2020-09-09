import { Client } from "discord.js";
import Web3 from "web3";

const TOKEN_DECIMAL = 18n;

const params = {
	// Discord app information
	DISCORD_TOKEN: process.env.DISCORD_TOKEN,
	DISCORD_CHANNEL: process.env.DISCORD_CHANNEL,

	// Web3 RPC access
	RPC_URL: process.env.RPC_URL,
	ACCOUNT_KEY: process.env.ACCOUNT_KEY,

	// Token distribution
	TOKEN_COUNT: BigInt(process.env.TOKEN_COUNT || 10),
}

Object.keys(params).forEach(param => {
	if (!params[param]) {
		console.log(`Missing ${param} env variables`);
		process.exit(1);
	}
})

const web3Api = new Web3(params.RPC_URL);

console.log(`Starting bot...`);
console.log(`Connecting web3 to ${params.RPC_URL}...`);

const client: Client = new Client();
const receivers: { [author: string]: number } = {};

client.on("ready", () => {
	console.log(`Logged in as ${client.user.tag}!`);
});

const onReceiveMessage = async (msg) => {
	const authorId = msg && msg.author && msg.author.id;
	const messageContent = msg && msg.content;
	const channelId = msg && msg.channel && msg.channel.id;

	if (!messageContent || !authorId || channelId != params.DISCORD_CHANNEL) {
		return;
	}

	if (messageContent.startsWith("!faucet send")) {
		if (receivers[authorId] > Date.now() - 3600 * 1000) {
			msg.reply("Already received token (limited to once per hour)");
			return;
		}
		let address = messageContent.slice("!faucet send".length).trim();
		if (address.startsWith("0x")) {
			address = address.slice(2);
		}
		if (address.length != 40) {
			msg.reply("Invalid address. Must be of 40 characters long");
			return;
		}
		receivers[authorId] = Date.now();

		await web3Api.eth.sendSignedTransaction(
			(
				await web3Api.eth.accounts.signTransaction(
					{
						value: `${params.TOKEN_COUNT * (10n**TOKEN_DECIMAL)}`,
						gasPrice: "0x01",
						gas: "0x21000",
						to: `0x${address}`,
					},
					params.ACCOUNT_KEY
				)
			).rawTransaction
		);
		const accountBalance = BigInt(await web3Api.eth.getBalance(`0x${address}`));

		msg.reply(`Sent ${params.TOKEN_COUNT} DEV tokens to 0x${address} (balance: ${accountBalance / (10n**TOKEN_DECIMAL)} DEV)`);
	}
	if (messageContent.startsWith("!balance")) {
		let address = messageContent.slice("!balance".length).trim();
		if (address.startsWith("0x")) {
			address = address.slice(2);
		}
		if (address.length != 40) {
			msg.reply("Invalid address. Must be of 40 characters long");
			return;
		}
		const accountBalance = BigInt(await web3Api.eth.getBalance(`0x${address}`));
		msg.reply(`0x${address} - balance: ${accountBalance / (10n**TOKEN_DECIMAL)} DEV`);
	}
};

client.on("message", onReceiveMessage);

client.login(params.DISCORD_TOKEN);
