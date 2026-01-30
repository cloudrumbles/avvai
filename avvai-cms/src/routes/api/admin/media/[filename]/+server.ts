import type { RequestHandler } from '@sveltejs/kit';
import { proxyAdmin } from '$lib/server/adminProxy';

export const DELETE: RequestHandler = async (event) => {
	return proxyAdmin(event, { path: `/admin/assets/media/${event.params.filename}` });
};
