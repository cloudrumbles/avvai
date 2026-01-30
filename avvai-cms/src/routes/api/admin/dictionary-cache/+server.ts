import type { RequestHandler } from '@sveltejs/kit';
import { proxyAdmin } from '$lib/server/adminProxy';

export const GET: RequestHandler = async (event) => {
	return proxyAdmin(event, { path: '/admin/dictionary-cache/list' });
};

export const POST: RequestHandler = async (event) => {
	const action = event.url.searchParams.get('action') ?? 'get';
	const path = action === 'upsert' ? '/admin/dictionary-cache/upsert' : '/admin/dictionary-cache/get';
	return proxyAdmin(event, { path });
};

export const DELETE: RequestHandler = async (event) => {
	return proxyAdmin(event, { path: '/admin/dictionary-cache/delete' });
};
