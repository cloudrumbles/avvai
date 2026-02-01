import { json } from '@sveltejs/kit';
import { fetchBackendJson } from '$lib/services/api-client';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async () => {
	const data = await fetchBackendJson('/lesson/list');
	return json(data);
};
