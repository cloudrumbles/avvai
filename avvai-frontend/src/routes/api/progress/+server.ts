import { json } from '@sveltejs/kit';
import { fetchBackend, fetchBackendJson, parseRequestJson } from '$lib/services/api-client';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async () => {
	const data = await fetchBackendJson('/progress/get');
	return json(data);
};

export const POST: RequestHandler = async ({ request }) => {
	const body = await parseRequestJson(request);

	await fetchBackend('/progress/update', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(body)
	});
	return json({ success: true });
};
