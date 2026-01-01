import { render } from 'solid-js/web'
import { createSignal, createMemo, onMount, Show } from 'solid-js'
import { LessonContainer } from '../shared/components'
import { Sidebar } from '../shared/components/Sidebar'
import { themes } from '../shared/styles/theme.css'
import * as styles from '../shared/styles/sidebar.css'
import type { Lesson } from '../shared/types'

type Section = { id: string; section: number; titleTamil: string }
type Chapter = { chapter: number; title: string; sections: Section[] }
type Manifest = {
  course: string
  courseTitle: string
  chapters: Chapter[]
}

function App() {
  const [manifest, setManifest] = createSignal<Manifest | null>(null)
  const [lesson, setLesson] = createSignal<Lesson | null>(null)
  const [currentId, setCurrentId] = createSignal<string>('cikaram-1-2')
  const [error, setError] = createSignal<string | null>(null)
  const [menuOpen, setMenuOpen] = createSignal(false)

  // Flatten all sections for navigation
  const allSections = createMemo(() => {
    const m = manifest()
    if (!m) return []
    return m.chapters.flatMap(ch => ch.sections)
  })

  const currentIndex = createMemo(() => {
    return allSections().findIndex(s => s.id === currentId())
  })

  const progress = createMemo(() => {
    const total = allSections().length
    if (total === 0) return 0
    return ((currentIndex() + 1) / total) * 100
  })

  const prevSection = createMemo(() => {
    const idx = currentIndex()
    return idx > 0 ? allSections()[idx - 1] : null
  })

  const nextSection = createMemo(() => {
    const idx = currentIndex()
    const all = allSections()
    return idx < all.length - 1 ? all[idx + 1] : null
  })

  const loadLesson = async (id: string) => {
    try {
      const res = await fetch(`/lessons/${id}.json`)
      if (!res.ok) throw new Error('Failed to load lesson')
      const data = await res.json()
      setLesson(data)
      setCurrentId(id)
      setMenuOpen(false)
      window.scrollTo(0, 0)
    } catch (e) {
      setError((e as Error).message)
    }
  }

  onMount(async () => {
    document.body.classList.add(themes['marutham-light'])

    try {
      const manifestRes = await fetch('/lessons/manifest.json')
      if (manifestRes.ok) {
        const manifestData = await manifestRes.json()
        setManifest(manifestData)
      }
      await loadLesson(currentId())
    } catch (e) {
      setError((e as Error).message)
    }
  })

  return (
    <>
      {/* Progress bar */}
      <div class={styles.progressBar}>
        <div class={styles.progressFill} style={{ width: `${progress()}%` }} />
      </div>

      {/* Mobile Header */}
      <header class={styles.mobileHeader}>
        <button class={styles.menuButton} onClick={() => setMenuOpen(true)}>
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="3" y1="6" x2="21" y2="6" />
            <line x1="3" y1="12" x2="21" y2="12" />
            <line x1="3" y1="18" x2="21" y2="18" />
          </svg>
        </button>
        <span class={styles.mobileTitle}>{lesson()?.titleTamil || 'Loading...'}</span>
      </header>

      {/* Overlay */}
      <Show when={menuOpen()}>
        <div class={styles.overlay} onClick={() => setMenuOpen(false)} />
      </Show>

      {/* Sidebar */}
      <Show when={manifest()}>
        <Sidebar
          manifest={manifest()!}
          currentLessonId={currentId()}
          onSelect={loadLesson}
          isOpen={menuOpen()}
          onClose={() => setMenuOpen(false)}
        />
      </Show>

      {/* Main Content */}
      <main class={styles.mainContent}>
        {error() && <p style={{ padding: '2rem', color: 'red' }}>Error: {error()}</p>}
        {lesson() && <LessonContainer lesson={lesson()!} />}
        {!lesson() && !error() && <p style={{ padding: '2rem' }}>Loading...</p>}
      </main>

      {/* Bottom Navigation */}
      <nav class={styles.bottomNav}>
        <button
          class={`${styles.navButton} ${!prevSection() ? styles.navButtonDisabled : ''}`}
          onClick={() => prevSection() && loadLesson(prevSection()!.id)}
          disabled={!prevSection()}
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="15 18 9 12 15 6" />
          </svg>
          Prev
        </button>
        <button
          class={`${styles.navButton} ${styles.navButtonPrimary} ${!nextSection() ? styles.navButtonDisabled : ''}`}
          onClick={() => nextSection() && loadLesson(nextSection()!.id)}
          disabled={!nextSection()}
        >
          Next
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="9 18 15 12 9 6" />
          </svg>
        </button>
      </nav>
    </>
  )
}

render(() => <App />, document.getElementById('app')!)
