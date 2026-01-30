import { env } from '$env/dynamic/private';
import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

const BASE = env.BACKEND_URL ?? 'http://localhost:3001';

export const POST: RequestHandler = async ({ request }) => {
	const { word } = await request.json();
	if (!word) return error(400, 'Missing word');

	const res = await fetch(`${BASE}/dictionary/lemmatise`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ word })
	});

	if (!res.ok) return error(res.status, 'Lemmatisation failed');

	const data = await res.json();
	return json(data);
};
