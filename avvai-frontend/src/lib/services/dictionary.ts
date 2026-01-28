export interface DictionaryEntry {
	word: string;
	definition: string;
	examples?: string[];
}

export async function lookup(word: string): Promise<DictionaryEntry | null> {
	const res = await fetch(`/api/dictionary?word=${encodeURIComponent(word)}`);
	if (!res.ok) return null;
	return res.json();
}
