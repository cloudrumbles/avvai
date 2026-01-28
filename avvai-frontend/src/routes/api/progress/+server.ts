import { env } from '$env/dynamic/private';
import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

const BASE = env.BACKEND_URL ?? 'http://localhost:3001';

export const GET: RequestHandler = async () => {
	const res = await fetch(`${BASE}/progress/get`);
	if (!res.ok) return error(res.status, 'Failed to fetch progress');

	const data = await res.json();
	return json(data);
};

export const POST: RequestHandler = async ({ request }) => {
	const body = await request.json();

	const res = await fetch(`${BASE}/progress/update`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(body)
	});

	if (!res.ok) return error(res.status, 'Failed to update progress');

	return json({ success: true });
};
