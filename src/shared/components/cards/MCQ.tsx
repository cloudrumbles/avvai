import * as styles from '../../styles/cards.css'
import type { MCQCard } from '../../types'
import { For, createSignal } from 'solid-js'

const labels = ['அ', 'ஆ', 'இ', 'ஈ', 'உ', 'ஊ']

export function MCQ(props: MCQCard) {
  const [selected, setSelected] = createSignal<number | null>(null)

  const handleSelect = (index: number) => {
    if (selected() !== null) return
    setSelected(index)
  }

  const getClass = (index: number) => {
    if (selected() === null) return styles.mcqOption
    if (index === props.correct) return `${styles.mcqOption} ${styles.mcqCorrect}`
    if (index === selected()) return `${styles.mcqOption} ${styles.mcqIncorrect}`
    return styles.mcqOption
  }

  return (
    <div class={styles.mcq}>
      <p class={styles.mcqQuestion}>{props.question}</p>
      <div class={styles.mcqOptions}>
        <For each={props.options}>
          {(option, i) => (
            <button
              class={getClass(i())}
              onClick={() => handleSelect(i())}
              data-answered={selected() !== null ? '' : undefined}
            >
              <span class={styles.mcqLabel}>{labels[i()]}</span>
              {option}
            </button>
          )}
        </For>
      </div>
    </div>
  )
}
