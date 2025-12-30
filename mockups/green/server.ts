import { Elysia, t } from 'elysia';
import { staticPlugin } from '@elysiajs/static';
import { Database } from 'bun:sqlite';
import { resolve, dirname } from 'path';

// Get the directory where this script lives
const __dirname = dirname(Bun.main);

// Initialize in-memory SQLite database
const db = new Database(':memory:');

// Create tables
db.run(`
  CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
  );

  CREATE TABLE lessons (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    difficulty TEXT NOT NULL
  );

  CREATE TABLE progress (
    user_id INTEGER NOT NULL,
    lesson_id INTEGER NOT NULL,
    completed BOOLEAN DEFAULT FALSE,
    score INTEGER DEFAULT 0,
    PRIMARY KEY (user_id, lesson_id)
  );
`);

// Seed lessons
const seedLessons = [
  { title: 'வணக்கம் - Greetings', content: 'Learn basic Tamil greetings and introductions.', difficulty: 'beginner' },
  { title: 'எண்கள் - Numbers', content: 'Master counting from 1 to 100 in Tamil.', difficulty: 'beginner' },
  { title: 'குடும்பம் - Family', content: 'Vocabulary for family members and relationships.', difficulty: 'beginner' },
  { title: 'உணவு - Food', content: 'Common food items and ordering at restaurants.', difficulty: 'intermediate' },
  { title: 'பயணம் - Travel', content: 'Essential phrases for traveling in Tamil Nadu.', difficulty: 'intermediate' },
  { title: 'வேலை - Work', content: 'Professional vocabulary and workplace conversations.', difficulty: 'intermediate' },
  { title: 'இலக்கணம் - Grammar', content: 'Advanced Tamil grammar structures.', difficulty: 'advanced' },
  { title: 'இலக்கியம் - Literature', content: 'Introduction to classical Tamil literature.', difficulty: 'advanced' },
];

const insertLesson = db.prepare('INSERT INTO lessons (title, content, difficulty) VALUES (?, ?, ?)');
for (const lesson of seedLessons) {
  insertLesson.run(lesson.title, lesson.content, lesson.difficulty);
}

// Session store
const sessions = new Map<string, number>(); // token -> user_id

function generateToken(): string {
  const bytes = new Uint8Array(32);
  crypto.getRandomValues(bytes);
  return Array.from(bytes).map(b => b.toString(16).padStart(2, '0')).join('');
}

// Quiz answer key
const quizAnswers: Record<number, { correct: string; nextPartial: string }> = {
  1: { correct: 'hello', nextPartial: 'quiz-correct.html' },
  2: { correct: 'thank_you', nextPartial: 'quiz-q2-correct.html' },
  3: { correct: 'eppadi_irukkeenga', nextPartial: 'quiz-complete.html' },
  4: { correct: 'poi_varugiren', nextPartial: 'quiz-complete.html' },
};

// Prepared statements
const findUserByEmail = db.prepare('SELECT id, email, password_hash, created_at FROM users WHERE email = ?');
const findUserById = db.prepare('SELECT id, email, created_at FROM users WHERE id = ?');
const insertUser = db.prepare('INSERT INTO users (email, password_hash) VALUES (?, ?)');
const getAllLessons = db.prepare('SELECT id, title, content, difficulty FROM lessons');
const getLessonById = db.prepare('SELECT id, title, content, difficulty FROM lessons WHERE id = ?');
const upsertProgress = db.prepare(`
  INSERT INTO progress (user_id, lesson_id, completed, score) VALUES (?, ?, true, ?)
  ON CONFLICT(user_id, lesson_id) DO UPDATE SET completed = true, score = excluded.score
`);
const getUserProgress = db.prepare('SELECT user_id, lesson_id, completed, score FROM progress WHERE user_id = ?');

const app = new Elysia()
  // Logging middleware
  .onRequest(({ request }) => {
    (request as any).startTime = performance.now();
  })
  .onAfterResponse(({ request }) => {
    const duration = performance.now() - (request as any).startTime;
    console.log(`${request.method} ${new URL(request.url).pathname} ${duration.toFixed(2)}ms`);
  })

  // ==================== AUTH ROUTES ====================

  // POST /api/register
  .post('/api/register', async ({ body }) => {
    const { email, password } = body as { email: string; password: string };

    const hash = await Bun.password.hash(password, "bcrypt");

    try {
      const result = insertUser.run(email, hash);
      const id = Number(result.lastInsertRowid);
      const token = generateToken();
      sessions.set(token, id);

      return { token, user: { id, email } };
    } catch (e) {
      return new Response(JSON.stringify({ error: 'Email already exists' }), {
        status: 409,
        headers: { 'Content-Type': 'application/json' }
      });
    }
  })

  // POST /api/login
  .post('/api/login', async ({ body }) => {
    const { email, password } = body as { email: string; password: string };

    const user = findUserByEmail.get(email) as { id: number; email: string; password_hash: string; created_at: string } | null;
    if (!user) {
      return new Response(JSON.stringify({ error: 'Invalid credentials' }), {
        status: 401,
        headers: { 'Content-Type': 'application/json' }
      });
    }

    const valid = await Bun.password.verify(password, user.password_hash, "bcrypt");
    if (!valid) {
      return new Response(JSON.stringify({ error: 'Invalid credentials' }), {
        status: 401,
        headers: { 'Content-Type': 'application/json' }
      });
    }

    const token = generateToken();
    sessions.set(token, user.id);

    return { token, user: { id: user.id, email: user.email, created_at: user.created_at } };
  })

  // GET /api/me
  .get('/api/me', ({ request }) => {
    const auth = request.headers.get('Authorization');
    if (!auth?.startsWith('Bearer ')) {
      return new Response(JSON.stringify({ error: 'Unauthorized' }), {
        status: 401,
        headers: { 'Content-Type': 'application/json' }
      });
    }

    const token = auth.slice(7);
    const userId = sessions.get(token);
    if (!userId) {
      return new Response(JSON.stringify({ error: 'Unauthorized' }), {
        status: 401,
        headers: { 'Content-Type': 'application/json' }
      });
    }

    const user = findUserById.get(userId) as { id: number; email: string; created_at: string } | null;
    if (!user) {
      return new Response(JSON.stringify({ error: 'User not found' }), {
        status: 404,
        headers: { 'Content-Type': 'application/json' }
      });
    }

    return user;
  })

  // ==================== LESSON ROUTES ====================

  // GET /api/lessons
  .get('/api/lessons', () => {
    return getAllLessons.all();
  })

  // GET /api/lessons/:id
  .get('/api/lessons/:id', ({ params }) => {
    const lesson = getLessonById.get(params.id);
    if (!lesson) {
      return new Response(JSON.stringify({ error: 'Lesson not found' }), {
        status: 404,
        headers: { 'Content-Type': 'application/json' }
      });
    }
    return lesson;
  })

  // POST /api/lessons/:id/complete
  .post('/api/lessons/:id/complete', ({ request, params, body }) => {
    const auth = request.headers.get('Authorization');
    if (!auth?.startsWith('Bearer ')) {
      return new Response(JSON.stringify({ error: 'Unauthorized' }), {
        status: 401,
        headers: { 'Content-Type': 'application/json' }
      });
    }

    const token = auth.slice(7);
    const userId = sessions.get(token);
    if (!userId) {
      return new Response(JSON.stringify({ error: 'Unauthorized' }), {
        status: 401,
        headers: { 'Content-Type': 'application/json' }
      });
    }

    const { score } = body as { score?: number };
    upsertProgress.run(userId, params.id, score ?? 0);

    return { status: 'ok' };
  })

  // GET /api/progress
  .get('/api/progress', ({ request }) => {
    const auth = request.headers.get('Authorization');
    if (!auth?.startsWith('Bearer ')) {
      return new Response(JSON.stringify({ error: 'Unauthorized' }), {
        status: 401,
        headers: { 'Content-Type': 'application/json' }
      });
    }

    const token = auth.slice(7);
    const userId = sessions.get(token);
    if (!userId) {
      return new Response(JSON.stringify({ error: 'Unauthorized' }), {
        status: 401,
        headers: { 'Content-Type': 'application/json' }
      });
    }

    return getUserProgress.all(userId);
  })

  // ==================== QUIZ ROUTE ====================

  .post('/check-answer', async ({ body }) => {
    const { question, answer } = body as { question: number; answer: string };

    const questionData = quizAnswers[question];
    if (!questionData) {
      return new Response('Invalid question', { status: 400 });
    }

    const isCorrect = answer === questionData.correct;
    const partialPath = isCorrect
      ? resolve(__dirname, 'partials', questionData.nextPartial)
      : resolve(__dirname, 'partials', 'quiz-incorrect.html');

    const partial = await Bun.file(partialPath).text();
    return new Response(partial, {
      headers: { 'Content-Type': 'text/html' },
    });
  })

  // ==================== STATIC FILES ====================

  .use(staticPlugin({
    assets: __dirname,
    prefix: '/',
  }))

  // Fallback for index
  .get('/', async () => {
    const html = await Bun.file(resolve(__dirname, 'index.html')).text();
    return new Response(html, {
      headers: { 'Content-Type': 'text/html' },
    });
  })

  .listen(3002);

console.log(`
  ┌─────────────────────────────────────────┐
  │                                         │
  │   ஆவ்வை - Tamil Learning (Bun)         │
  │                                         │
  │   Server running at:                    │
  │   http://localhost:${app.server?.port}                  │
  │                                         │
  └─────────────────────────────────────────┘
`);
