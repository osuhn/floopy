import { ApplyOptions } from '@sapphire/decorators';
import { Listener } from '@sapphire/framework';
import { GatewayDispatchEvents } from 'discord.js';
import type { VoiceServerUpdate } from 'lavaclient';

@ApplyOptions<Listener.Options>({
	name: GatewayDispatchEvents.VoiceServerUpdate,
	emitter: 'ws'
})
export class UserListener extends Listener {
	public override run(data: VoiceServerUpdate) {
		return this.container.client.music.handleVoiceUpdate(data);
	}
}
