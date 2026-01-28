import { env } from '$env/dynamic/private';
import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

const BASE = env.BACKEND_URL ?? 'http://localhost:3001';

export const GET: RequestHandler = async ({ url }) => {
	const word = url.searchParams.get('word');
	if (!word) return error(400, 'Missing word parameter');

	const res = await fetch(`${BASE}/dictionary/lookup?word=${encodeURIComponent(word)}`);
	if (!res.ok) return error(res.status, 'Dictionary lookup failed');

	const data = await res.json();
	return json(data);
};
