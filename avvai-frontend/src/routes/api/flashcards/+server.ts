import { env } from '$env/dynamic/private';
import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

const BASE = env.BACKEND_URL ?? 'http://localhost:3001';

export const GET: RequestHandler = async ({ url }) => {
	const limit = url.searchParams.get('limit');
	const res = await fetch(`${BASE}/flashcards/due${limit ? `?limit=${limit}` : ''}`);
	if (!res.ok) return error(res.status, 'Failed to fetch flashcards');

	const data = await res.json();
	return json(data);
};

export const POST: RequestHandler = async ({ request }) => {
	const body = await request.json();
	const res = await fetch(`${BASE}/flashcards/review`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(body)
	});

	if (!res.ok) return error(res.status, 'Failed to review flashcard');

	const data = await res.json();
	return json(data);
};
