import * as styles from '../../styles/cards.css'
import type { FillBlankCard } from '../../types'
import { For, createSignal } from 'solid-js'

export function FillBlank(props: FillBlankCard) {
  const [selected, setSelected] = createSignal<string | null>(null)
  const [checked, setChecked] = createSignal(false)

  const handleSelect = (word: string) => {
    if (checked()) return
    setSelected(word)
    setChecked(true)
  }

  const isCorrect = () => selected() === props.answer

  const blankClass = () => {
    if (!checked()) return styles.fillBlankBlank
    return `${styles.fillBlankBlank} ${isCorrect() ? styles.fillBlankCorrect : styles.fillBlankIncorrect}`
  }

  const parts = props.sentence.split('____________')

  return (
    <div class={styles.fillBlank}>
      <p class={styles.fillBlankSentence}>
        {parts[0]}
        <span class={blankClass()}>
          {selected() && <span class={styles.fillBlankFilled}>{selected()}</span>}
        </span>
        {parts[1]}
      </p>
      <div class={styles.fillBlankWordBank}>
        <For each={props.wordBank}>
          {(word) => (
            <button
              class={`${styles.fillBlankWord} ${selected() === word ? styles.fillBlankUsed : ''}`}
              onClick={() => handleSelect(word)}
              data-used={selected() === word ? '' : undefined}
            >
              {word}
            </button>
          )}
        </For>
      </div>
    </div>
  )
}
