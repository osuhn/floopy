import { Command, RegisterBehavior } from '@sapphire/framework';

export class UserCommand extends Command {
	public override registerApplicationCommands(registry: Command.Registry) {
		registry.registerChatInputCommand(
			(builder) =>
				builder //
					.setName('play')
					.setDescription('Play a song')
					.addStringOption((option) =>
						option //
							.setName('song')
							.setDescription('The song to play')
							.setRequired(true)
					),
			{
				behaviorWhenNotIdentical: RegisterBehavior.Overwrite
			}
		);
	}

	public override async chatInputRun(interaction: Command.ChatInputCommandInteraction<'cached'>) {
		const { player: musicPlayer, music } = this.container.client;
		const player = music.players.get(interaction.guildId) || music.createPlayer(interaction.guildId);

		const song = interaction.options.getString('song', true);

		if (!player.connected) player.connect(interaction.member.voice.channelId, { deafened: true });
		const { tracks } = await musicPlayer.findSpotifyTracks(song);

		console.log(tracks);

		player.queue.add(tracks[0], { requester: interaction.user.id, next: true });
		await player.queue.start();

		return interaction.reply(`Done`);
	}
}
