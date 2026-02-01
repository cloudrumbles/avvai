import { json } from '@sveltejs/kit';
import { fetchBackendJson, parseRequestJson } from '$lib/services/api-client';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async () => {
	const data = await fetchBackendJson('/flashcards/settings');
	return json(data);
};

export const POST: RequestHandler = async ({ request }) => {
	const body = await parseRequestJson(request);
	const data = await fetchBackendJson('/flashcards/settings', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(body)
	});
	return json(data);
};
