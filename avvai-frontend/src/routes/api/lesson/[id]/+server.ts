import { env } from '$env/dynamic/private';
import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

const BASE = env.BACKEND_URL ?? 'http://localhost:3001';

export const GET: RequestHandler = async ({ params }) => {
	const res = await fetch(`${BASE}/lesson/get?id=${encodeURIComponent(params.id)}`);
	if (!res.ok) return error(res.status, 'Failed to fetch lesson');

	const data = await res.json();
	return json(data);
};
