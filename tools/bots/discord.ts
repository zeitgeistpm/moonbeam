import { Client } from 'discord.js';

const DISCORD_TOKEN = process.env.DISCORD_TOKEN;
const DISCORD_CHANNEL = process.env.DISCORD_CHANNEL;
const TOKEN_COUNT = process.env.TOKEN_COUNT || 10;

if (!DISCORD_TOKEN || !DISCORD_CHANNEL || !TOKEN_COUNT) {
	console.log(`Missing DISCORD_TOKEN, DISCORD_CHANNEL or TOKEN_COUNT env variables`);
	process.exit(1);
}


console.log(`Starting bot...`);

const client: Client = new Client();
const receivers: {[author: string]: number} = {};

client.on('ready', () => {
  console.log(`Logged in as ${client.user.tag}!`);
});

client.on('message', msg => {
	const authorId = msg && msg.author && msg.author.id;
	const messageContent = msg && msg.content;
	const channelId = msg && msg.channel && msg.channel.id;

	if (!messageContent || !authorId || channelId != DISCORD_CHANNEL) {
		return;
	}

	if (messageContent.startsWith("!faucet send")) {
		if (receivers[authorId] > Date.now() - (3600 * 1000)) {
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
		receivers[authorId] =  Date.now();
		msg.reply(`Sent ${TOKEN_COUNT} DEV tokens to ${address}`);
	}
  console.log(msg);
});

client.login(DISCORD_TOKEN);
