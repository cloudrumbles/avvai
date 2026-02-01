import { json } from '@sveltejs/kit';
import { fetchBackendJson } from '$lib/services/api-client';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ params }) => {
	const data = await fetchBackendJson(`/lesson/get?id=${encodeURIComponent(params.id)}`);
	return json(data);
};
