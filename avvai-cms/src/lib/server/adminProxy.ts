import type { RequestEvent } from '@sveltejs/kit';

type ProxyOptions = {
	path: string;
};

const DEFAULT_BACKEND = 'http://localhost:3001';

function backendBase() {
	return process.env.BACKEND_URL ?? DEFAULT_BACKEND;
}

export async function proxyAdmin(event: RequestEvent, options: ProxyOptions) {
	const { session } = await event.locals.safeGetSession();

	const headers = new Headers();
	const contentType = event.request.headers.get('content-type');

	if (session?.access_token) {
		headers.set('authorization', `Bearer ${session.access_token}`);
	}
	if (contentType) {
		headers.set('content-type', contentType);
	}

	const url = `${backendBase()}${options.path}`;
	const method = event.request.method;

	const init: RequestInit & { duplex?: 'half' } = {
		method,
		headers
	};

	if (!['GET', 'HEAD'].includes(method)) {
		init.body = event.request.body;
		init.duplex = 'half';
	}

	const response = await fetch(url, init);
	const body = await response.arrayBuffer();

	return new Response(body, {
		status: response.status,
		headers: response.headers
	});
}
