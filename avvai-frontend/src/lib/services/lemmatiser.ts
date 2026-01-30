export interface LemmatiseResponse {
	lemma: string;
}

export async function lemmatise(word: string): Promise<LemmatiseResponse | null> {
	const res = await fetch('/api/dictionary/lemmatise', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ word })
	});
	if (!res.ok) return null;
	return res.json();
}
