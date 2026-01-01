import * as styles from '../../styles/cards.css'
import type { DialogueCard } from '../../types'
import { For } from 'solid-js'

export function Dialogue(props: DialogueCard) {
  return (
    <div class={styles.dialogue}>
      {props.context && <p class={styles.dialogueContext}>{props.context}</p>}
      <div class={styles.dialogueLines}>
        <For each={props.speakers}>
          {(speaker) => (
            <div class={styles.dialogueLine}>
              <span class={styles.dialogueSpeaker}>{speaker.name}</span>
              <span class={styles.dialogueText}>{speaker.line}</span>
            </div>
          )}
        </For>
      </div>
    </div>
  )
}
