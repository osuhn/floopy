import { CLIENT_OPTIONS } from '#root/config';
import { container, SapphireClient } from '@sapphire/framework';
import { MusicPlayer } from '#lib/structures/Player';
import { Node } from 'lavaclient';

export class FloopyClient extends SapphireClient {
	public override dev: boolean = process.env.NODE_ENV === 'development';

	public override readonly music: Node = new Node({
		sendGatewayPayload: (id, payload) => {
			const guild = this.guilds.cache.get(id);
			if (guild) guild.shard.send(payload);
		},
		connection: {
			host: process.env.LAVA_HOST,
			password: process.env.LAVA_PASS,
			port: Number(process.env.LAVA_PORT)
		}
	});

	public override readonly player: MusicPlayer = new MusicPlayer(this);

	public constructor() {
		super(CLIENT_OPTIONS);
	}

	public override async login(token: string = process.env.DISCORD_TOKEN) {
		const result = await super.login(token);
		return result;
	}

	public override async destroy() {
		await container.prisma.$disconnect();
		return super.destroy();
	}
}
