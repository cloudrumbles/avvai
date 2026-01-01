// Card types - discriminated union

export type ReadingCard = {
  type: 'reading'
  title?: string
  content: string
  source?: string
}

export type PoetryCard = {
  type: 'poetry'
  lines: string[]
  translation?: string
  source?: string
}

export type VocabCard = {
  type: 'vocab'
  title?: string
  items: { term: string; meaning: string }[]
}

export type MCQCard = {
  type: 'mcq'
  question: string
  options: string[]
  correct: number
}

export type FillBlankCard = {
  type: 'fill-blank'
  sentence: string
  answer: string
  wordBank: string[]
}

export type ShortAnswerCard = {
  type: 'short-answer'
  question: string
}

export type DialogueCard = {
  type: 'dialogue'
  speakers: { name: string; line: string }[]
  context?: string
}

// Union of all card types
export type Card =
  | ReadingCard
  | PoetryCard
  | VocabCard
  | MCQCard
  | FillBlankCard
  | ShortAnswerCard
  | DialogueCard

// Lesson structure
export type Lesson = {
  id: string
  title: string
  titleTamil: string
  chapter: number
  section: number
  cards: Card[]
}
