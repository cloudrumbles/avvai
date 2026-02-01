import type { Meta, StoryObj } from '@storybook/svelte';
import LessonList from '../../lib/components/lesson/LessonList.svelte';
import { lessonSummaries, lessonProgress } from '../mocks/lesson';

const meta = {
	title: 'Lesson/LessonList',
	component: LessonList,
	parameters: {
		layout: 'fullscreen'
	},
	args: {
		lessons: lessonSummaries,
		progress: lessonProgress,
		onselect: () => {}
	}
} satisfies Meta<LessonList>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Empty: Story = {
	args: {
		lessons: [],
		progress: {}
	}
};
