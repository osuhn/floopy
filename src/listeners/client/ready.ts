import { ApplyOptions } from '@sapphire/decorators';
import { Listener, Piece, Store } from '@sapphire/framework';
import { envParseString } from '@skyra/env-utilities';
import { blue, gray, green, magenta, magentaBright, white, yellow, red } from 'colorette';

const dev = envParseString('NODE_ENV') !== 'production';

@ApplyOptions<Listener.Options>({ once: true })
export class UserEvent extends Listener {
	private readonly style = dev ? yellow : blue;

	public async run() {
		this.container.client.music.connect(this.container.client.id!);
		await this.printBanner();
		this.printStoreDebugInformation();
	}

	private async printBanner() {
		const dbConnected = await this.container.prisma.$queryRaw`SELECT 1`;
		const success = green('+');
		const failed = red('-');

		const llc = dev ? magentaBright : white;
		const blc = dev ? magenta : blue;

		const line02 = llc('');
		const line03 = llc('');
		const line04 = llc('');
		const line05 = llc('');

		// Offset Pad
		const pad = ' '.repeat(7);

		console.log(
			String.raw`
${line02} ${pad}[${success}] Gateway
${line03} ${pad}[${this.container.client.music.conn.active ? success : failed}] Audio
${line04} ${pad}[${dbConnected ? success : failed}] Database
${line05}${dev ? ` ${pad}${blc('<')}${llc('/')}${blc('>')} ${llc('DEVELOPMENT MODE')}` : ''}
		`.trim()
		);
	}

	private printStoreDebugInformation() {
		const { client, logger } = this.container;
		const stores = [...client.stores.values()];

		for (const store of stores) {
			logger.info(this.styleStore(store));
		}
	}

	private styleStore(store: Store<Piece>) {
		return gray(`├─ Loaded ${this.style(store.size.toString().padEnd(3, ' '))} ${store.name}.`);
	}
}
