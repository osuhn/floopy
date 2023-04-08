import { ApplyOptions } from '@sapphire/decorators';
import { Command } from '@sapphire/framework';
import type { Message } from 'discord.js';

@ApplyOptions<Command.Options>({
	description: 'Ping the bot',
	aliases: ['p'],
	preconditions: ['OwnerOnly']
})
export class UserCommand extends Command {
	public override async messageRun(message: Message<true>) {
		const msg = await message.reply('Pinging...');

		const { diff, ping, dbPing } = await this.getPing(msg, message);

		return msg.edit(`Pong! Took ${diff}ms. Heartbeat: ${ping}ms. Database: ${dbPing}ms.`);
	}

	private async getPing(message: Message, message2: Message) {
		const diff = (message.editedTimestamp || message.createdTimestamp) - message2.createdTimestamp;
		const ping = Math.round(this.container.client.ws.ping);

		const start = Date.now();
		await this.container.prisma.$queryRaw`SELECT 1`;
		const dbPing = Date.now() - start;

		return { diff, ping, dbPing };
	}
}
