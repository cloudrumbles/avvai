import * as styles from '../../styles/cards.css'
import type { ShortAnswerCard } from '../../types'

export function ShortAnswer(props: ShortAnswerCard) {
  return (
    <div class={styles.shortAnswer}>
      <p class={styles.shortAnswerQuestion}>{props.question}</p>
      <p class={styles.shortAnswerNote}>இந்தக் கேள்விக்கு நீங்களே விடை எழுதுக.</p>
    </div>
  )
}
