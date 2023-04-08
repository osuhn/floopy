import { ApplyOptions } from '@sapphire/decorators';
import { Listener } from '@sapphire/framework';
import { GatewayDispatchEvents } from 'discord.js';
import type { VoiceStateUpdate } from 'lavaclient';

@ApplyOptions<Listener.Options>({
	name: GatewayDispatchEvents.VoiceServerUpdate,
	emitter: 'ws'
})
export class UserListener extends Listener {
	public override run(data: VoiceStateUpdate) {
		return this.container.client.music.handleVoiceUpdate(data);
	}
}
