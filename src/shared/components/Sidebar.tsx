import { For } from 'solid-js'
import * as styles from '../styles/sidebar.css'

type Section = {
  id: string
  section: number
  titleTamil: string
}

type Chapter = {
  chapter: number
  title: string
  sections: Section[]
}

type Manifest = {
  course: string
  courseTitle: string
  chapters: Chapter[]
}

type Props = {
  manifest: Manifest
  currentLessonId: string
  onSelect: (id: string) => void
  isOpen?: boolean
  onClose?: () => void
}

export function Sidebar(props: Props) {
  const sidebarClass = () => {
    const classes = [styles.sidebar]
    if (props.isOpen) classes.push(styles.sidebarOpen)
    return classes.join(' ')
  }

  return (
    <aside class={sidebarClass()}>
      <header class={styles.sidebarHeader}>
        <h2 class={styles.courseTitle}>{props.manifest.courseTitle}</h2>
        <button class={styles.closeButton} onClick={props.onClose}>
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>
      </header>
      <nav class={styles.sidebarNav}>
        <For each={props.manifest.chapters}>
          {(chapter) => (
            <div class={styles.chapter}>
              <h3 class={styles.chapterTitle}>{chapter.title}</h3>
              <ul class={styles.sectionList}>
                <For each={chapter.sections}>
                  {(section) => (
                    <li>
                      <button
                        class={`${styles.sectionItem} ${props.currentLessonId === section.id ? styles.sectionActive : ''}`}
                        onClick={() => props.onSelect(section.id)}
                      >
                        <span class={styles.sectionNumber}>{section.section}</span>
                        <span class={styles.sectionTitle}>{section.titleTamil}</span>
                      </button>
                    </li>
                  )}
                </For>
              </ul>
            </div>
          )}
        </For>
      </nav>
    </aside>
  )
}
