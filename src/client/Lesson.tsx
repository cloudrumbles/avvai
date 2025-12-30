import { createSignal, For, Show, type Component, createEffect } from 'solid-js';
import * as s from './lesson.css';
import { lessonData, type Page, type Section, type VocabularyItem } from './lessonData';

// Components

const Term: Component<{ children: any }> = (props) => (
  <span class={s.term}>{props.children}</span>
);

const VocabGrid: Component<{ items: VocabularyItem[] }> = (props) => (
  <div class={s.vocabGrid}>
    <For each={props.items}>{(item) => (
      <div class={s.vocabCard}>
        <div class={s.vocabTamil}>{item.tamil}</div>
        <div class={s.vocabMeta}>
          <div class={s.vocabEnglish}>{item.english}</div>
          <Show when={item.phonetic}>
            <div class={s.vocabPhonetic}>/{item.phonetic}/</div>
          </Show>
          <Show when={item.extra}>
            <div class={s.vocabExtra}>{item.extra}</div>
          </Show>
        </div>
      </div>
    )}</For>
  </div>
);

const SectionRenderer: Component<{ section: Section, sectionIndex: number }> = (props) => {
  const [isOpen, setIsOpen] = createSignal(true);

  // Determine section type style
  const typeClass = () => {
    switch (props.section.type) {
      case 'text': return s.sectionTypeContent;
      case 'vocabulary': return s.sectionTypeVocab;
      case 'exercise': return s.sectionTypeQuiz;
      default: return s.sectionTypeContent;
    }
  };

  const typeLabel = () => {
    switch (props.section.type) {
      case 'text': return 'Text';
      case 'vocabulary': return 'Vocab';
      case 'exercise': return 'Quiz';
      case 'examples': return 'Ex';
      default: return 'Info';
    }
  };

  return (
    <div class={s.section}>
      <div class={s.sectionCard}>
        <div class={s.sectionHeader} onClick={() => setIsOpen(!isOpen())}>
          <span class={`${s.sectionType} ${typeClass()}`}>{typeLabel()}</span>
          <h2 class={s.sectionTitle}>{props.section.heading}</h2>
          <span class={`${s.sectionToggle} ${isOpen() ? s.sectionToggleOpen : ''}`}>
            &#9660;
          </span>
        </div>
        <div class={`${s.sectionBody} ${isOpen() ? s.sectionBodyOpen : ''}`}>
          <Show when={props.section.content}>
            <p class={s.tamilBlock}>{props.section.content}</p>
          </Show>
          
          <Show when={props.section.items && props.section.items.length > 0}>
            <Show when={props.section.type === 'vocabulary'} fallback={
               <div class={s.inlineVocabList}>
                 <For each={props.section.items}>{(item) => (
                   <div class={s.inlineVocabItem}>
                     <span class={s.inlineVocabTamil}>{item.tamil}</span>
                     <span class={s.inlineVocabEnglish}> - {item.english}</span>
                   </div>
                 )}</For>
               </div>
            }>
              <VocabGrid items={props.section.items} />
            </Show>
          </Show>
        </div>
      </div>
    </div>
  );
};

const PageRenderer: Component<{ page: Page }> = (props) => {
  return (
    <div class={s.pageContainer}>
      <header class={s.lessonHeader}>
        <div class={s.lessonMeta}>
          <span class={s.lessonBadge}>{lessonData.lesson_id.toUpperCase()}</span>
          <span class={s.pageIndicator}>Page {props.page.page_number} of {lessonData.pages.length}</span>
        </div>
        <h1 class={s.lessonTitle}>{props.page.title}</h1>
        <Show when={props.page.summary}>
          <p class={s.pageSummary}>{props.page.summary}</p>
        </Show>
      </header>

      <For each={props.page.sections}>{(section, i) => (
        <SectionRenderer section={section} sectionIndex={i()} />
      )}</For>
    </div>
  );
};

const Lesson: Component = () => {
  const [currentPageIndex, setCurrentPageIndex] = createSignal(0);
  
  const currentPage = () => lessonData.pages[currentPageIndex()];
  const progress = () => ((currentPageIndex() + 1) / lessonData.pages.length) * 100;

  const goToPage = (index: number) => {
    if (index >= 0 && index < lessonData.pages.length) {
      setCurrentPageIndex(index);
      window.scrollTo({ top: 0, behavior: 'smooth' });
    }
  };

  return (
    <div class={s.lessonLayout}>
      <div class={s.kolam} />

      <aside class={s.sidebar}>
        <div class={s.sidebarHeader}>
          <div class={s.courseId}>{lessonData.course_id.toUpperCase()}</div>
          <div class={s.courseTitle}>{lessonData.title}</div>
          <div class={s.courseSubtitle}>{lessonData.subtitle}</div>
        </div>

        <nav class={s.navSections}>
          <div class={s.navGroupLabel}>Table of Contents</div>
          <For each={lessonData.pages}>{(page, i) => (
            <a
              class={`${s.navItem} ${i() === currentPageIndex() ? s.navItemActive : ''} ${i() < currentPageIndex() ? s.navItemCompleted : ''}`}
              onClick={() => goToPage(i())}
            >
              <Show when={i() < currentPageIndex()}>
                <span class={s.checkMark}>&#10003;</span>
              </Show>
              <Show when={i() >= currentPageIndex()}>
                <span class={s.navItemNum}>{page.page_number}</span>
              </Show>
              {page.title.split('(')[0]} {/* Truncate for cleaner sidebar */}
            </a>
          )}</For>
        </nav>
      </aside>

      <main class={s.lessonMain}>
        <div class={s.progressBar}>
           <div class={s.progressFill} style={{ width: `${progress()}%` }} />
        </div>

        <PageRenderer page={currentPage()} />

        <nav class={s.pageNav}>
          <button 
            class={s.pageNavBtn} 
            disabled={currentPageIndex() === 0}
            onClick={() => goToPage(currentPageIndex() - 1)}
            style={{ opacity: currentPageIndex() === 0 ? 0.5 : 1, cursor: currentPageIndex() === 0 ? 'not-allowed' : 'pointer' }}
          >
            <span class={s.pageNavArrow}>&larr;</span>
            <div class={s.pageNavContent}>
              <div class={s.pageNavLabel}>Previous</div>
              <div class={s.pageNavTitle}>
                {currentPageIndex() > 0 ? lessonData.pages[currentPageIndex() - 1].title.split('(')[0] : 'Start'}
              </div>
            </div>
          </button>

          <button 
            class={`${s.pageNavBtn} ${s.pageNavBtnNext}`}
            disabled={currentPageIndex() === lessonData.pages.length - 1}
            onClick={() => goToPage(currentPageIndex() + 1)}
            style={{ opacity: currentPageIndex() === lessonData.pages.length - 1 ? 0.5 : 1, cursor: currentPageIndex() === lessonData.pages.length - 1 ? 'not-allowed' : 'pointer' }}
          >
            <div class={s.pageNavContent}>
              <div class={s.pageNavLabel}>Next</div>
              <div class={s.pageNavTitle}>
                {currentPageIndex() < lessonData.pages.length - 1 ? lessonData.pages[currentPageIndex() + 1].title.split('(')[0] : 'Finish'}
              </div>
            </div>
            <span class={s.pageNavArrow}>&rarr;</span>
          </button>
        </nav>
      </main>
    </div>
  );
};

export default Lesson;
