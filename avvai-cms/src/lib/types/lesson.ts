// Mirrors avvai-frontend lesson types

export interface LessonSummary {
	id: string;
	title: string;
	description: string;
}

export interface Lesson {
	id: string;
	title: string;
	description: string;
	source?: SourceMetadata;
	sections: ContentSection[];
}

export interface SourceMetadata {
	title: string;
	author?: string;
	period?: string;
	verse_count?: number;
	poetic_form?: string;
}

export type ContentSection =
	| ({ type: 'prose' } & ProseSection)
	| ({ type: 'poetry' } & PoetrySection)
	| ({ type: 'vocabulary' } & VocabularySection)
	| ({ type: 'exercises' } & ExercisesSection)
	| ({ type: 'media' } & MediaSection)
	| ({ type: 'dialogue' } & DialogueSection);

export interface ProseSection {
	title?: string;
	paragraphs: string[];
}

export interface PoetrySection {
	title?: string;
	verses: Verse[];
}

export interface Verse {
	number?: number;
	lines: string[];
	translation?: string;
}

export interface VocabularySection {
	title?: string;
	entries: VocabularyEntry[];
}

export interface DialogueSection {
	title?: string;
	scene?: SceneInfo;
	lines: DialogueLine[];
}

export interface SceneInfo {
	location?: string;
	time?: string;
	characters?: string[];
}

export interface DialogueLine {
	character?: string;
	text: string;
	direction?: boolean;
}

export interface VocabularyEntry {
	word: string;
	meaning: string;
}

export interface ExercisesSection {
	title?: string;
	exercise_groups: ExerciseGroup[];
}

export interface ExerciseGroup {
	group_type: ExerciseGroupType;
	instructions: string;
	exercises: Exercise[];
}

export type ExerciseGroupType = 'multiple_choice' | 'fill_in_blank' | 'short_answer' | 'long_answer';

export interface Exercise {
	id: string;
	content: ExerciseContent;
}

export type ExerciseContent =
	| ({ exercise_type: 'multiple_choice' } & MultipleChoiceExercise)
	| ({ exercise_type: 'fill_in_blank' } & FillInBlankExercise)
	| ({ exercise_type: 'short_answer' } & ShortAnswerExercise)
	| ({ exercise_type: 'long_answer' } & LongAnswerExercise);

export interface MultipleChoiceExercise {
	question: string;
	options: ChoiceOption[];
}

export interface ChoiceOption {
	id: string;
	text: string;
	correct: boolean;
}

export interface FillInBlankExercise {
	text_before: string;
	text_after?: string;
	accepted_answers: string[];
	options?: string[];
	hint?: string;
}

export interface ShortAnswerExercise {
	question: string;
	model_answer?: string;
}

export interface LongAnswerExercise {
	question: string;
	model_answer?: string;
	min_words?: number;
}

export interface MediaSection {
	title?: string;
	media_type: MediaType;
	url: string;
	caption?: string;
}

export type MediaType = 'image' | 'audio' | 'video';
