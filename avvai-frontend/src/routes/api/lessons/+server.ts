import { env } from '$env/dynamic/private';
import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

const BASE = env.BACKEND_URL ?? 'http://localhost:3001';

export const GET: RequestHandler = async () => {
	const res = await fetch(`${BASE}/lesson/list`);
	if (!res.ok) return error(res.status, 'Failed to fetch lessons');

	const data = await res.json();
	return json(data);
};
