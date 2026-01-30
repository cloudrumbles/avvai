import { env } from '$env/dynamic/private';
import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

const BASE = env.BACKEND_URL ?? 'http://localhost:3001';

export const GET: RequestHandler = async () => {
	const res = await fetch(`${BASE}/flashcards/settings`);
	if (!res.ok) return error(res.status, 'Failed to fetch flashcard settings');

	const data = await res.json();
	return json(data);
};

export const POST: RequestHandler = async ({ request }) => {
	const body = await request.json();
	const res = await fetch(`${BASE}/flashcards/settings`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(body)
	});

	if (!res.ok) return error(res.status, 'Failed to update flashcard settings');

	const data = await res.json();
	return json(data);
};
