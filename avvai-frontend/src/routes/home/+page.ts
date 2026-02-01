import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
	const [lessonsRes, progressRes] = await Promise.all([
		fetch('/api/lessons'),
		fetch('/api/progress')
	]);

	const lessons = lessonsRes.ok ? await lessonsRes.json() : [];
	const progress = progressRes.ok ? await progressRes.json() : {};

	return { lessons, progress };
};
