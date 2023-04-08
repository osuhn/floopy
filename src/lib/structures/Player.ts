import { load, SpotifyItemType } from '@lavaclient/spotify';
import type { Addable } from '@lavaclient/queue';
import { LoadType } from '@lavaclient/types/v3';
import type { FloopyClient } from '#lib/Floopy';

export class MusicPlayer {
	public constructor(private readonly client: FloopyClient) {
		if (process.env.SPOTIFY_CLIENT_SECRET && process.env.SPOTIFY_CLIENT_ID) {
			load({
				client: {
					id: process.env.SPOTIFY_CLIENT_ID!,
					secret: process.env.SPOTIFY_CLIENT_SECRET!
				},
				autoResolveYoutubeTracks: true
			});
		}
	}

	public async findTracks(query: string): Promise<{ tracks: Addable[]; type: LoadType }> {
		const results = await this.client.music.rest.loadTracks(/^https?:\/\//.test(query) ? query : `ytsearch:${query}`);

		let tracks: Addable[] = [];
		switch (results.loadType) {
			case LoadType.LoadFailed:
			case LoadType.NoMatches:
				throw new Error('No tracks found.');
			case LoadType.PlaylistLoaded:
				tracks = results.tracks;
				break;
			case LoadType.TrackLoaded:
			case LoadType.SearchResult: {
				const [track] = results.tracks;
				tracks = [track];
				break;
			}
		}

		return { tracks, type: results.loadType };
	}

	public async findSpotifyTracks(query: string): Promise<{ tracks: Addable[]; type: SpotifyItemType }> {
		if (!this.client.music.spotify.isSpotifyUrl(query)) {
			throw new Error('Spotify is not loaded');
		}

		const item = await this.client.music.spotify.load(query);

		if (!item) throw new Error('No tracks found');
		let tracks: Addable[] = [];

		switch (item.type) {
			case SpotifyItemType.Track:
				tracks = [await item.resolveYoutubeTrack()];
				break;
			case SpotifyItemType.Album:
			case SpotifyItemType.Artist:
			case SpotifyItemType.Playlist:
				tracks = await item.resolveYoutubeTracks();
				break;
		}

		return { tracks, type: item.type };
	}
}
