import { env } from '$env/dynamic/private';
import { error } from '@sveltejs/kit';

const BASE = env.BACKEND_URL ?? 'http://localhost:3001';

export async function parseRequestJson<T>(request: Request): Promise<T> {
	try {
		return (await request.json()) as T;
	} catch {
		throw error(400, 'Invalid JSON body');
	}
}

export async function fetchBackend(path: string, init?: RequestInit): Promise<Response> {
	let res: Response;
	try {
		res = await fetch(`${BASE}${path}`, init);
	} catch {
		throw error(503, 'Backend unavailable');
	}

	if (!res.ok) {
		const status = res.status >= 500 ? 503 : res.status;
		const message = res.status >= 500 ? 'Backend unavailable' : 'Backend request failed';
		throw error(status, message);
	}

	return res;
}

export async function fetchBackendJson<T>(path: string, init?: RequestInit): Promise<T> {
	const res = await fetchBackend(path, init);
	try {
		return (await res.json()) as T;
	} catch {
		throw error(503, 'Invalid response from backend');
	}
}
