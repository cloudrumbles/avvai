import type { RequestHandler } from '@sveltejs/kit';
import { proxyAdmin } from '$lib/server/adminProxy';

export const GET: RequestHandler = async (event) => {
	return proxyAdmin(event, { path: `/admin/content/lessons/${event.params.id}` });
};

export const PUT: RequestHandler = async (event) => {
	return proxyAdmin(event, { path: `/admin/content/lessons/${event.params.id}` });
};

export const DELETE: RequestHandler = async (event) => {
	return proxyAdmin(event, { path: `/admin/content/lessons/${event.params.id}` });
};
