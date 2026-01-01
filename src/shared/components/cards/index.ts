import type { Component } from 'solid-js'
import type { Card } from '../../types'

import { Reading } from './Reading'
import { Poetry } from './Poetry'
import { Vocab } from './Vocab'
import { MCQ } from './MCQ'
import { FillBlank } from './FillBlank'
import { ShortAnswer } from './ShortAnswer'
import { Dialogue } from './Dialogue'

// Card component registry
export const cards: Record<Card['type'], Component<any>> = {
  'reading': Reading,
  'poetry': Poetry,
  'vocab': Vocab,
  'mcq': MCQ,
  'fill-blank': FillBlank,
  'short-answer': ShortAnswer,
  'dialogue': Dialogue,
}

export { Reading, Poetry, Vocab, MCQ, FillBlank, ShortAnswer, Dialogue }
