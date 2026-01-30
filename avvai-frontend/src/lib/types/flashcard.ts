export interface Flashcard {
	id: string;
	front: string;
	back: string;
}

export interface ReviewResponse {
	success: boolean;
	due_date: string;
	interval_days: number;
}

export interface FlashcardSettings {
	desired_retention: number;
}
