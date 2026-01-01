import * as styles from '../../styles/cards.css'
import type { VocabCard } from '../../types'
import { For } from 'solid-js'

export function Vocab(props: VocabCard) {
  return (
    <div class={styles.vocab}>
      <p class={styles.vocabTitle}>{props.title ?? 'பொருள் அறிவோம்'}</p>
      <ul class={styles.vocabList}>
        <For each={props.items}>
          {(item) => (
            <li class={styles.vocabItem}>
              <span class={styles.vocabTerm}>{item.term}</span>
              <span class={styles.vocabMeaning}>{item.meaning}</span>
            </li>
          )}
        </For>
      </ul>
    </div>
  )
}
