import { Elysia } from "elysia";

export const api = new Elysia({ prefix: '/api' })
    .get('/health', () => ({ status: 'ok', timestamp: Date.now() }))
    .get('/lessons', () => [
        { id: 1, title: 'Introduction to Vowels', completed: true },
        { id: 2, title: 'Consonants Part 1', completed: false }
    ]);
