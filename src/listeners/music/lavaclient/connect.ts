import { ApplyOptions } from '@sapphire/decorators';
import { Listener } from '@sapphire/framework';

@ApplyOptions<Listener.Options>({
	emitter: 'music'
})
export class UserListener extends Listener {
	public override run() {
		this.container.logger.info('Connected to lavalink');
	}
}
