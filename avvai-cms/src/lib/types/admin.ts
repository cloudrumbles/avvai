export interface DictionaryEntry {
	word: string;
	definition: string;
	examples: string[];
}

export interface DictionaryCacheEntry {
	key: string;
	entry: DictionaryEntry;
}

export interface LessonListItem {
	id: string;
	title: string;
	description: string;
	filename: string;
}
