import * as styles from '../../styles/cards.css'
import type { PoetryCard } from '../../types'
import { For } from 'solid-js'

export function Poetry(props: PoetryCard) {
  return (
    <div class={styles.poetry}>
      <For each={props.lines}>
        {(line) => <p class={styles.poetryLine}>{line}</p>}
      </For>
      {props.translation && <p class={styles.poetryTranslation}>{props.translation}</p>}
      {props.source && <p class={styles.poetrySource}>— {props.source}</p>}
    </div>
  )
}
