// =============================================================================
// LESSON TYPES - Mirrors Rust backend data model
// =============================================================================

/** Summary shown in lesson list (doesn't include full content) */
export interface LessonSummary {
  id: string;
  title: string;
  description: string;
}

/** Full lesson with all content sections */
export interface Lesson {
  id: string;
  title: string;
  description: string;
  source?: SourceMetadata;
  sections: ContentSection[];
}

/** Metadata about the literary source */
export interface SourceMetadata {
  title: string;
  author?: string;
  period?: string;
  verse_count?: number;
  poetic_form?: string;
}

// =============================================================================
// CONTENT SECTIONS - Discriminated union by `type` field
// =============================================================================

/**
 * A lesson is composed of multiple sections, each with a `type` discriminator:
 * - `prose`: Paragraphs of explanatory text
 * - `poetry`: Tamil verses with optional translations
 * - `vocabulary`: Word definitions in a grid layout
 * - `exercises`: Interactive practice questions
 * - `media`: Images, audio, or video content
 * - `dialogue`: Drama/play dialogue with characters and stage directions
 */
export type ContentSection =
  | ({ type: "prose" } & ProseSection)
  | ({ type: "poetry" } & PoetrySection)
  | ({ type: "vocabulary" } & VocabularySection)
  | ({ type: "exercises" } & ExercisesSection)
  | ({ type: "media" } & MediaSection)
  | ({ type: "dialogue" } & DialogueSection);

/** Explanatory text paragraphs */
export interface ProseSection {
  title?: string;
  paragraphs: string[];
}

/** Tamil poetry with verses */
export interface PoetrySection {
  title?: string;
  verses: Verse[];
}

/** A single verse of poetry */
export interface Verse {
  /** Verse number (e.g., 1, 2, 3) */
  number?: number;
  /** Lines of the verse in Tamil */
  lines: string[];
  /** English/Tamil translation */
  translation?: string;
}

/** Word definitions displayed in a grid */
export interface VocabularySection {
  title?: string;
  entries: VocabularyEntry[];
}

// =============================================================================
// DIALOGUE SECTION (for drama/plays)
// =============================================================================

/** Drama/play dialogue with scene info and character lines */
export interface DialogueSection {
  title?: string;
  scene?: SceneInfo;
  lines: DialogueLine[];
}

/** Scene metadata for dialogue sections */
export interface SceneInfo {
  /** Location (e.g., "காடு", "அரண்மனை") */
  location?: string;
  /** Time of day (e.g., "காலை", "மாலை") */
  time?: string;
  /** Characters in the scene */
  characters?: string[];
}

/** A single line of dialogue or stage direction */
export interface DialogueLine {
  /** Character name (omit for stage directions) */
  character?: string;
  /** The dialogue text */
  text: string;
  /** Stage direction (e.g., "வெளியேறுகிறார்", "திரை விழுகிறது") */
  direction?: boolean;
}

/** A vocabulary word and its meaning */
export interface VocabularyEntry {
  word: string;
  meaning: string;
}

// =============================================================================
// EXERCISES
// =============================================================================

/** Container for exercise groups */
export interface ExercisesSection {
  title?: string;
  exercise_groups: ExerciseGroup[];
}

/** A group of exercises with shared instructions */
export interface ExerciseGroup {
  group_type: ExerciseGroupType;
  /** Instructions shown above the group (e.g., "சரியான விடையைத் தேர்ந்தெடுக்க") */
  instructions: string;
  exercises: Exercise[];
}

export type ExerciseGroupType =
  | "multiple_choice"
  | "fill_in_blank"
  | "short_answer"
  | "long_answer";

/** A single exercise with unique ID */
export interface Exercise {
  id: string;
  content: ExerciseContent;
}

/**
 * Exercise content discriminated by `exercise_type`:
 * - `multiple_choice`: Pick one correct answer from options
 * - `fill_in_blank`: Complete the sentence (dropdown if options provided, text input otherwise)
 * - `short_answer`: Free-form text with optional model answer reveal
 * - `long_answer`: Extended response with optional word minimum
 */
export type ExerciseContent =
  | ({ exercise_type: "multiple_choice" } & MultipleChoiceExercise)
  | ({ exercise_type: "fill_in_blank" } & FillInBlankExercise)
  | ({ exercise_type: "short_answer" } & ShortAnswerExercise)
  | ({ exercise_type: "long_answer" } & LongAnswerExercise);

/** Pick one correct answer from multiple options */
export interface MultipleChoiceExercise {
  question: string;
  options: ChoiceOption[];
}

/** An option in a multiple choice question */
export interface ChoiceOption {
  /** Option identifier (e.g., "a", "b", "c", "d") */
  id: string;
  text: string;
  correct: boolean;
}

/**
 * Fill in the blank exercise.
 * If `options` is provided OR `hint` matches pattern "(a, b, c)", renders as dropdown.
 * Otherwise renders as text input.
 */
export interface FillInBlankExercise {
  /** Text before the blank */
  text_before: string;
  /** Text after the blank */
  text_after?: string;
  /** Correct answers (case-insensitive matching) */
  accepted_answers: string[];
  /** Dropdown options. If absent, parsed from hint if hint matches "(a, b, c)" pattern */
  options?: string[];
  /** Hint text, or word bank in format "(option1, option2, option3)" */
  hint?: string;
}

/** Free-form short text response */
export interface ShortAnswerExercise {
  question: string;
  /** Revealed when user clicks "Show answer" */
  model_answer?: string;
}

/** Extended response exercise */
export interface LongAnswerExercise {
  question: string;
  /** Revealed when user clicks "Show answer" */
  model_answer?: string;
  /** Minimum word count hint shown to user */
  min_words?: number;
}

// =============================================================================
// MEDIA
// =============================================================================

/** Embedded media content */
export interface MediaSection {
  media_type: MediaType;
  url: string;
  caption?: string;
}

export type MediaType = "image" | "audio" | "video";
