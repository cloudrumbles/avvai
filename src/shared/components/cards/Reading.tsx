import * as styles from '../../styles/cards.css'
import type { ReadingCard } from '../../types'

export function Reading(props: ReadingCard) {
  return (
    <div class={styles.reading}>
      {props.title && <h3 class={styles.cardTitle}>{props.title}</h3>}
      <p innerHTML={props.content} />
      {props.source && <p class={styles.readingSource}>— {props.source}</p>}
    </div>
  )
}
