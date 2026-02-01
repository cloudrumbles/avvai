import { lemmatise } from './lemmatiser';

export interface DictionaryEntry {
	word: string;
	definition: string;
	examples?: string[];
}

const CACHE_KEY = 'dictionary-cache-v1';
const MAX_ENTRIES = 500;

type CacheEntry = {
	value: DictionaryEntry | null;
	time: number;
};

type DictionaryCache = Record<string, CacheEntry>;

let inMemoryCache: DictionaryCache | null = null;

function normalise(word: string) {
	return word.trim().toLowerCase();
}

function loadCache(): DictionaryCache {
	if (inMemoryCache) return inMemoryCache;
	if (typeof localStorage === 'undefined') return {};

	try {
		const raw = localStorage.getItem(CACHE_KEY);
		inMemoryCache = raw ? (JSON.parse(raw) as DictionaryCache) : {};
	} catch {
		inMemoryCache = {};
	}

	return inMemoryCache;
}

function saveCache(cache: DictionaryCache) {
	if (typeof localStorage === 'undefined') return;

	try {
		localStorage.setItem(CACHE_KEY, JSON.stringify(cache));
	} catch {
		// ignore storage errors (quota, privacy mode)
	}
}

function enforceLimit(cache: DictionaryCache) {
	const entries = Object.entries(cache);
	if (entries.length <= MAX_ENTRIES) return;

	entries.sort((a, b) => a[1].time - b[1].time);
	const toRemove = entries.length - MAX_ENTRIES;
	for (let i = 0; i < toRemove; i += 1) {
		delete cache[entries[i][0]];
	}
}

async function fetchEntry(word: string): Promise<DictionaryEntry | null> {
	const res = await fetch('/api/dictionary', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ word })
	});
	return res.ok ? ((await res.json()) as DictionaryEntry) : null;
}

export async function lookup(word: string): Promise<DictionaryEntry | null> {
	const key = normalise(word);
	if (!key) return null;

	const cache = loadCache();
	const cached = cache[key];
	if (cached?.value) return cached.value;

	let value = await fetchEntry(key);

	if (!value) {
		const lemmaResponse = await lemmatise(key);
		const lemma = normalise(lemmaResponse?.lemma ?? '');
		if (lemma && lemma !== key) {
			const lemmaCached = cache[lemma];
			value = lemmaCached ? lemmaCached.value : await fetchEntry(lemma);
			if (value && !lemmaCached) {
				cache[lemma] = { value, time: Date.now() };
			}
		}
	}

	cache[key] = { value, time: Date.now() };
	enforceLimit(cache);
	saveCache(cache);

	return value;
}
