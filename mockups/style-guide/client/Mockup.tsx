import { createSignal, For, Show } from 'solid-js';
import * as css from './mockup.css';

// ============================================
// DATA
// ============================================

const lessons = [
  { num: 1, title: 'Greetings', tamil: 'வாழ்த்துகள்', duration: 15, level: 'Beginner' },
  { num: 2, title: 'Numbers', tamil: 'எண்கள்', duration: 20, level: 'Beginner' },
  { num: 3, title: 'Family', tamil: 'குடும்பம்', duration: 25, level: 'Beginner' },
  { num: 4, title: 'Food & Drink', tamil: 'உணவும் பானமும்', duration: 30, level: 'Intermediate' },
];

const vocab = [
  { num: 1, tamil: 'வணக்கம்', translit: 'vanakkam', meaning: 'Hello / Greetings' },
  { num: 2, tamil: 'நன்றி', translit: 'nandri', meaning: 'Thank you' },
  { num: 3, tamil: 'எப்படி இருக்கீங்க?', translit: 'eppadi irukkeenga?', meaning: 'How are you?' },
  { num: 4, tamil: 'நல்லா இருக்கேன்', translit: 'nalla irukken', meaning: 'I am fine' },
  { num: 5, tamil: 'போய் வருகிறேன்', translit: 'poi varugiren', meaning: 'Goodbye' },
  { num: 6, tamil: 'வாங்க', translit: 'vaanga', meaning: 'Welcome' },
];

const thinaiList = [
  { tamil: 'மருதம்', name: 'Marutham', color: '#558b2f' },
  { tamil: 'பாலை', name: 'Palai', color: '#d4a574' },
  { tamil: 'நெய்தல்', name: 'Neithal', color: '#8d76a0' },
  { tamil: 'முல்லை', name: 'Mullai', color: '#fb8c00' },
  { tamil: 'குறிஞ்சி', name: 'Kurinji', color: '#6c5ce7' },
];

// ============================================
// ICONS
// ============================================

const ArrowIcon = () => (
  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
    <path d="M5 12h14M12 5l7 7-7 7"/>
  </svg>
);

const BackIcon = () => (
  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
    <path d="M19 12H5M12 19l-7-7 7-7"/>
  </svg>
);

const ClockIcon = () => (
  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
    <circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/>
  </svg>
);

const StarIcon = () => (
  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
    <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
  </svg>
);

// ============================================
// HEADER
// ============================================

function Header(props: { onNav: (p: 'home' | 'lesson') => void }) {
  return (
    <header class={css.header}>
      <div class={css.logo} onClick={() => props.onNav('home')}>
        <div class={css.logoMark}>ஆ</div>
        <span class={css.logoText}>ஆவ்வை</span>
      </div>
      <nav class={css.nav}>
        <span class={css.navLink} onClick={() => props.onNav('home')}>முகப்பு</span>
        <span class={css.navLink} onClick={() => props.onNav('lesson')}>பாடங்கள்</span>
        <span class={css.navLinkDisabled}>பயிற்சி</span>
      </nav>
    </header>
  );
}

// ============================================
// FOOTER
// ============================================

function Footer() {
  return (
    <footer class={css.footer}>
      <p class={css.textMuted} style={{ 'margin-bottom': '0.5rem' }}>Built with love for Tamil</p>
      <p class={`${css.textPrimary} ${css.fontTamil}`} style={{ 'font-size': '1.1rem' }}>அறிவே ஆற்றல்</p>
    </footer>
  );
}

// ============================================
// HOME PAGE
// ============================================

function HomePage(props: { onNav: (p: 'home' | 'lesson') => void }) {
  return (
    <div class={css.flexGrow}>
      {/* Hero */}
      <section class={css.hero}>
        <div class={css.heroContent}>
          <p class={css.heroGreeting}>வணக்கம்!</p>
          <h1 class={css.heroTitle}>Learn Tamil the Natural Way</h1>
          <p class={css.heroSubtitle}>
            Immerse yourself in the ancient beauty of Tamil through the lens of Sangam poetry.
          </p>
          <div class={css.buttonGroup}>
            <button class={css.btnPrimary} onClick={() => props.onNav('lesson')}>
              Start Learning <ArrowIcon />
            </button>
            <button class={css.btnSecondary}>Browse Lessons</button>
          </div>
        </div>
      </section>

      {/* Progress */}
      <section class={css.section}>
        <div class={`${css.card} ${css.progressCard}`}>
          <div class={css.progressHeader}>
            <span>Your Progress</span>
            <span>3 of 12</span>
          </div>
          <div class={css.progressTrack}>
            <div class={css.progressFill} style={{ width: '25%' }}></div>
          </div>
        </div>
      </section>

      {/* Lessons Grid */}
      <section class={css.sectionNoPadTop}>
        <h2 class={css.sectionTitle}>Featured Lessons</h2>
        <div class={css.grid}>
          <For each={lessons}>{(lesson) => (
            <div
              class={`${css.card} ${css.cardClickable} ${css.lessonCard}`}
              onClick={() => props.onNav('lesson')}
            >
              <span class={css.lessonCardLabel}>Lesson {lesson.num}</span>
              <h3 class={css.lessonCardTitle}>{lesson.title}</h3>
              <p class={css.lessonCardTamil}>{lesson.tamil}</p>
              <div class={css.lessonCardMeta}>
                <span class={css.metaItem}><ClockIcon /> {lesson.duration}m</span>
                <span class={css.metaItem}><StarIcon /> {lesson.level}</span>
              </div>
            </div>
          )}</For>
        </div>
      </section>

      {/* Thinai */}
      <section class={css.sectionNoPadTop} style={{ 'padding-bottom': '3rem' }}>
        <div class={`${css.card} ${css.thinaiCard}`}>
          <h2 class={css.thinaiTitle}>
            <span class={`${css.textPrimary} ${css.fontTamil}`}>திணை</span> — The Five Landscapes
          </h2>
          <div class={css.thinaiList}>
            <For each={thinaiList}>{(t) => (
              <div class={css.thinaiItem}>
                <span class={css.thinaiDot} style={{ background: t.color }}></span>
                <span class={css.thinaiName}>{t.tamil}</span>
                <span class={css.thinaiNameEn}>({t.name})</span>
              </div>
            )}</For>
          </div>
        </div>
      </section>
    </div>
  );
}

// ============================================
// LESSON PAGE
// ============================================

function LessonPage(props: { onNav: (p: 'home' | 'lesson') => void }) {
  const [answered, setAnswered] = createSignal<string | null>(null);

  return (
    <div class={css.lessonContainer}>
      {/* Back */}
      <a class={css.backLink} onClick={() => props.onNav('home')}>
        <BackIcon /> Back to lessons
      </a>

      {/* Title */}
      <div class={css.lessonHeader}>
        <div class={css.badgeGroup}>
          <span class={`${css.badge} ${css.badgeSecondary}`}>Lesson 1</span>
          <span class={`${css.badge} ${css.badgePrimary}`}>Beginner</span>
        </div>
        <h1 class={css.lessonTitle}>வாழ்த்துகள்</h1>
        <h2 class={css.lessonSubtitle}>Greetings</h2>
      </div>

      {/* Vocab */}
      <section style={{ 'margin-bottom': '2rem' }}>
        <h3 class={css.sectionTitle}>Vocabulary</h3>
        <div class={`${css.grid} ${css.gridNarrow}`}>
          <For each={vocab}>{(w) => (
            <div class={`${css.card} ${css.vocabCard}`}>
              <p class={css.vocabTamil}>{w.tamil}</p>
              <p class={css.vocabTranslit}>{w.translit}</p>
              <p class={css.vocabMeaning}>{w.meaning}</p>
            </div>
          )}</For>
        </div>
      </section>

      {/* Grammar */}
      <section style={{ 'margin-bottom': '2rem' }}>
        <h3 class={css.sectionTitle}>Grammar Notes</h3>
        <div class={css.grammarBox}>
          <h4 class={css.grammarTitle}>Formal vs Informal</h4>
          <p class={css.grammarText}>
            Tamil uses different verb endings for formal and informal speech. Add
            <span class={`${css.textPrimary} ${css.fontTamil}`}> -ங்க </span>
            (-nga) for respect.
          </p>
          <div class={css.grammarExample}>
            <span class={`${css.grammarTamil} ${css.fontTamil}`}>எப்படி இருக்கீங்க?</span>
            <span class={css.grammarLabel}>(formal)</span>
          </div>
          <div class={css.grammarExample}>
            <span class={`${css.grammarTamil} ${css.fontTamil}`}>எப்படி இருக்க?</span>
            <span class={css.grammarLabel}>(informal)</span>
          </div>
        </div>
      </section>

      {/* Quiz */}
      <section style={{ 'margin-bottom': '2rem' }}>
        <h3 class={css.sectionTitle}>Quick Quiz</h3>
        <div class={css.quizBox}>
          <Show when={!answered()}>
            <p class={css.quizPrompt}>What does this mean?</p>
            <p class={css.quizWord}>வணக்கம்</p>
            <div class={css.quizOptions}>
              <button class={css.quizOption} onClick={() => setAnswered('wrong')}>Goodbye</button>
              <button class={css.quizOption} onClick={() => setAnswered('correct')}>Hello</button>
              <button class={css.quizOption} onClick={() => setAnswered('wrong')}>Thank you</button>
              <button class={css.quizOption} onClick={() => setAnswered('wrong')}>How are you?</button>
            </div>
          </Show>
          <Show when={answered()}>
            <div class={css.quizResult}>
              <div class={css.quizEmoji}>{answered() === 'correct' ? '🎉' : '📚'}</div>
              <p class={css.quizResultTitle}>{answered() === 'correct' ? 'Correct!' : 'Not quite!'}</p>
              <p class={css.quizResultText}>வணக்கம் means "Hello / Greetings"</p>
              <button class={css.btnPrimary} onClick={() => setAnswered(null)}>Try Again</button>
            </div>
          </Show>
        </div>
      </section>

      {/* Nav */}
      <div class={css.navButtons}>
        <button class={css.btnSecondary} onClick={() => props.onNav('home')}>
          <BackIcon /> Back
        </button>
        <button class={css.btnPrimary}>
          Next <ArrowIcon />
        </button>
      </div>
    </div>
  );
}

// ============================================
// MAIN MOCKUP
// ============================================

export function Mockup() {
  const [page, setPage] = createSignal<'home' | 'lesson'>('home');

  return (
    <div class={css.mockupContainer}>
      <Header onNav={setPage} />
      <Show when={page() === 'home'}>
        <HomePage onNav={setPage} />
      </Show>
      <Show when={page() === 'lesson'}>
        <LessonPage onNav={setPage} />
      </Show>
      <Footer />
    </div>
  );
}
