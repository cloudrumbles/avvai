import { renderToString } from 'solid-js/web'
import { LessonContainer } from '../shared/components'
import { themes } from '../shared/styles/theme.css'
import type { Lesson } from '../shared/types'

// Read lesson from JSON file
async function getLesson(id: string): Promise<Lesson | null> {
  const path = `./data/lessons/${id}.json`
  const file = Bun.file(path)
  if (!(await file.exists())) return null
  return file.json()
}

// HTML shell
function htmlShell(content: string, data: Lesson, theme = 'marutham-light') {
  return `<!DOCTYPE html>
<html lang="ta">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>${data.titleTamil} - Avvai</title>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Catamaran:wght@400;600;700&family=Crimson+Pro:ital,wght@0,400;0,600;1,400&display=swap" rel="stylesheet">
</head>
<body class="${themes[theme as keyof typeof themes]}">
  <div id="app">${content}</div>
  <script>
    window.__LESSON_DATA__ = ${JSON.stringify(data)};
  </script>
  <script type="module" src="/src/client/index.tsx"></script>
</body>
</html>`
}

// Server
Bun.serve({
  port: 3000,
  async fetch(req) {
    const url = new URL(req.url)

    // Lesson route
    const lessonMatch = url.pathname.match(/^\/lesson\/(.+)$/)
    if (lessonMatch) {
      const id = lessonMatch[1]
      const lesson = await getLesson(id)

      if (!lesson) {
        return new Response('Lesson not found', { status: 404 })
      }

      const html = renderToString(() => <LessonContainer lesson={lesson} />)
      const page = htmlShell(html, lesson)

      return new Response(page, {
        headers: { 'Content-Type': 'text/html' },
      })
    }

    // Home
    if (url.pathname === '/') {
      return new Response('Avvai - Tamil Learning', {
        headers: { 'Content-Type': 'text/html' },
      })
    }

    return new Response('Not found', { status: 404 })
  },
})

console.log('Server running at http://localhost:3000')
