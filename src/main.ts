import { FloopyClient } from '#lib/Floopy';
import { container } from '@sapphire/framework';

import '#lib/setup';

const client = new FloopyClient();

const main = async () => {
	try {
		await client.login();
	} catch (error) {
		container.logger.error(error);
		client.destroy();
		process.exit(1);
	}
};

main().catch(container.logger.error.bind(container.logger));
