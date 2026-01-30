import { createServerClient } from '@supabase/ssr';
import { type Handle, redirect } from '@sveltejs/kit';
import { env } from '$env/dynamic/public';

const PUBLIC_PATHS = ['/login'];
const AUTH_DISABLED = true;

function isPublicPath(pathname: string) {
	if (pathname.startsWith('/_app') || pathname.startsWith('/favicon')) {
		return true;
	}
	return PUBLIC_PATHS.some((path) => pathname === path || pathname.startsWith(path + '/'));
}

function parseAllowlist(raw: string | undefined) {
	return (raw ?? '')
		.split(',')
		.map((email) => email.trim().toLowerCase())
		.filter(Boolean);
}

export const handle: Handle = async ({ event, resolve }) => {
	const supabaseUrl = env.PUBLIC_SUPABASE_URL;
	const supabaseKey = env.PUBLIC_SUPABASE_PUBLISHABLE_DEFAULT_KEY;

	if (!supabaseUrl || !supabaseKey) {
		return new Response('Auth misconfigured', { status: 500 });
	}

	event.locals.supabase = createServerClient(supabaseUrl, supabaseKey, {
		cookies: {
			getAll: () => event.cookies.getAll(),
			setAll: (cookiesToSet) => {
				cookiesToSet.forEach(({ name, value, options }) => {
					try {
						event.cookies.set(name, value, { ...options, path: '/' });
					} catch {
						/* ignore cookie errors after response sent */
					}
				});
			}
		}
	});

	event.locals.safeGetSession = async () => {
		const {
			data: { session }
		} = await event.locals.supabase.auth.getSession();

		if (!session) {
			return { session: null, user: null };
		}

		const {
			data: { user },
			error
		} = await event.locals.supabase.auth.getUser();

		if (error) {
			return { session: null, user: null };
		}

		return { session, user };
	};

	if (AUTH_DISABLED) {
		return resolve(event);
	}

	if (isPublicPath(event.url.pathname)) {
		return resolve(event);
	}

	const { session, user } = await event.locals.safeGetSession();

	if (!session || !user) {
		redirect(302, '/login');
	}

	const allowlist = parseAllowlist(env.PUBLIC_ADMIN_EMAILS);
	const email = user.email?.toLowerCase() ?? '';

	if (!email || !allowlist.includes(email)) {
		return new Response('Forbidden - email not in allowlist', { status: 403 });
	}

	event.locals.user = user;

	return resolve(event);
};
