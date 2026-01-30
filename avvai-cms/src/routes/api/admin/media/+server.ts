import type { RequestHandler } from '@sveltejs/kit';
import { proxyAdmin } from '$lib/server/adminProxy';

export const GET: RequestHandler = async (event) => {
	return proxyAdmin(event, { path: '/admin/assets/media' });
};

export const POST: RequestHandler = async (event) => {
	return proxyAdmin(event, { path: '/admin/assets/media' });
};
