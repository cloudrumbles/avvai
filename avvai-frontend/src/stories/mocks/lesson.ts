import type {
	ContentSection,
	Lesson,
	LessonSummary,
	ExercisesSection
} from '../../lib/types/lesson';

export const proseSection: ContentSection = {
	type: 'prose',
	title: 'Context',
	paragraphs: [
		'In the ancient poems, landscapes carry emotion. Each setting shapes the mood of the story.',
		'This short passage introduces the idea of place as a character in the text.'
	]
};

export const poetrySection: ContentSection = {
	type: 'poetry',
	title: 'Verse Fragment',
	verses: [
		{
			number: 1,
			lines: ['The hill breeze arrives at dusk', 'Carrying the scent of rain'],
			translation: 'Evening wind brings the promise of rain.'
		},
		{
			number: 2,
			lines: ['A quiet path, a lantern glow', 'Footsteps fade into the night']
		}
	]
};

export const vocabularySection: ContentSection = {
	type: 'vocabulary',
	title: 'Key Words',
	entries: [
		{ word: 'kurinji', meaning: 'A mountainous landscape, often tied to love.' },
		{ word: 'mullai', meaning: 'A forest landscape, linked with patience.' },
		{ word: 'marutham', meaning: 'Farmland landscape, linked with daily life.' },
		{ word: 'neithal', meaning: 'Seashore landscape, linked with longing.' }
	]
};

export const dialogueSection: ContentSection = {
	type: 'dialogue',
	title: 'Scene',
	scene: {
		location: 'Village square',
		time: 'Evening',
		characters: ['Anjali', 'Kannan']
	},
	lines: [
		{ character: 'Anjali', text: 'Do you hear the temple bell?' },
		{ character: 'Kannan', text: 'Yes. It means the day is closing.' },
		{ direction: true, text: 'They walk toward the light.' }
	]
};

export const mediaSection: ContentSection = {
	type: 'media',
	media_type: 'image',
	url: 'https://placehold.co/960x540?text=Landscape+Study',
	caption: 'Landscape study for a lesson section.'
};

export const exercisesSection: ExercisesSection & { type: 'exercises' } = {
	type: 'exercises',
	title: 'Practice',
	exercise_groups: [
		{
			group_type: 'multiple_choice',
			instructions: 'Choose the best meaning.',
			exercises: [
				{
					id: 'mcq-1',
					content: {
						exercise_type: 'multiple_choice',
						question: 'What does "kurinji" describe?',
						options: [
							{ id: 'a', text: 'Mountain landscape', correct: true },
							{ id: 'b', text: 'Seashore landscape', correct: false },
							{ id: 'c', text: 'Farmland landscape', correct: false }
						]
					}
				}
			]
		},
		{
			group_type: 'fill_in_blank',
			instructions: 'Complete the sentence.',
			exercises: [
				{
					id: 'fib-1',
					content: {
						exercise_type: 'fill_in_blank',
						text_before: 'The forest landscape is called',
						text_after: '.',
						accepted_answers: ['mullai'],
						hint: '(kurinji, mullai, neithal)'
					}
				}
			]
		},
		{
			group_type: 'short_answer',
			instructions: 'Write a short response.',
			exercises: [
				{
					id: 'short-1',
					content: {
						exercise_type: 'short_answer',
						question: 'How does setting affect mood?',
						model_answer: 'It creates atmosphere and signals emotion.'
					}
				}
			]
		},
		{
			group_type: 'long_answer',
			instructions: 'Write a longer response.',
			exercises: [
				{
					id: 'long-1',
					content: {
						exercise_type: 'long_answer',
						question: 'Describe a scene using one of the landscape themes.',
						min_words: 40,
						model_answer: 'A calm forest evening with soft light and quiet sounds.'
					}
				}
			]
		}
	]
};

export const contentSections: ContentSection[] = [
	proseSection,
	poetrySection,
	vocabularySection,
	dialogueSection,
	mediaSection,
	exercisesSection
];

export const lessonSummaries: LessonSummary[] = [
	{
		id: 'lesson-1',
		title: 'Landscapes and Meaning',
		description: 'How settings shape tone and emotion.'
	},
	{
		id: 'lesson-2',
		title: 'Poetic Images',
		description: 'Close reading of imagery and voice.'
	},
	{
		id: 'lesson-3',
		title: 'Dialogue and Scene',
		description: 'Characters in motion and conversation.'
	}
];

export const lessonProgress: Record<string, boolean> = {
	'lesson-1': true,
	'lesson-2': false,
	'lesson-3': false
};

export const fullLesson: Lesson = {
	id: 'lesson-2',
	title: 'Poetic Images',
	description: 'Close reading of imagery and voice.',
	sections: contentSections
};
