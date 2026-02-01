import { json, error } from '@sveltejs/kit';
import { fetchBackendJson, parseRequestJson } from '$lib/services/api-client';
import type { RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ request }) => {
	const { word } = await parseRequestJson<{ word?: string }>(request);
	if (!word) return error(400, 'Missing word');
	const data = await fetchBackendJson('/dictionary/lemmatise', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ word })
	});
	return json(data);
};
