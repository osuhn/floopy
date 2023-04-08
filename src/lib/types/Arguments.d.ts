import type { ArrayString, NumberString } from '@skyra/env-utilities';
import type { PrismaClient } from '@prisma/client';
import type { MusicPlayer } from '#lib/structures/Player';
import type { Node } from 'lavaclient';

declare module 'discord.js' {
	interface Client {
		dev: boolean;
		music: Node;
		player: MusicPlayer;
	}
}

declare module '@sapphire/framework' {
	interface Preconditions {
		OwnerOnly: never;
		ReviewerOnly: never;
	}
}

declare module '@skyra/env-utilities' {
	interface Env {
		CLIENT_OWNERS: ArrayString;
		DISCORD_TOKEN: string;

		LAVA_HOST: string;
		LAVA_PASS: string;
		LAVA_PORT: NumberString;

		DATABASE_URL: string;
	}
}
declare module '@sapphire/pieces' {
	interface Container {
		prisma: PrismaClient;
	}
}

export default undefined;
