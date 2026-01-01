import * as styles from '../styles/cards.css'
import type { Lesson } from '../types'
import { LessonCard } from './LessonCard'
import { For } from 'solid-js'

type Props = {
  lesson: Lesson
}

export function LessonContainer(props: Props) {
  return (
    <article class={styles.lesson}>
      <header class={styles.lessonHeader}>
        <p class={styles.lessonChapter}>பாடம் {props.lesson.chapter}.{props.lesson.section}</p>
        <h1 class={styles.lessonTitle}>{props.lesson.titleTamil}</h1>
      </header>
      <div class={styles.lessonCards}>
        <For each={props.lesson.cards}>
          {(card) => <LessonCard card={card} />}
        </For>
      </div>
    </article>
  )
}
