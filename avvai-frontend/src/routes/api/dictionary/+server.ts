import { json, error } from '@sveltejs/kit';
import { fetchBackendJson, parseRequestJson } from '$lib/services/api-client';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request }) => {
	const { word } = await parseRequestJson<{ word?: string }>(request);
	if (!word) return error(400, 'Missing word parameter');
	const data = await fetchBackendJson('/dictionary/lookup', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ word })
	});
	return json(data);
};
