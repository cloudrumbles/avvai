import { json } from '@sveltejs/kit';
import { fetchBackendJson, parseRequestJson } from '$lib/services/api-client';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ url }) => {
	const limit = url.searchParams.get('limit');
	const data = await fetchBackendJson(`/flashcards/due${limit ? `?limit=${limit}` : ''}`);
	return json(data);
};

export const POST: RequestHandler = async ({ request }) => {
	const body = await parseRequestJson(request);
	const data = await fetchBackendJson('/flashcards/review', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(body)
	});
	return json(data);
};
