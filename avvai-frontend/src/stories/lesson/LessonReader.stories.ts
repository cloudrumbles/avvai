import type { Meta, StoryObj } from '@storybook/svelte';
import LessonReader from '../../lib/components/lesson/LessonReader.svelte';
import { fullLesson } from '../mocks/lesson';

const meta = {
	title: 'Lesson/LessonReader',
	component: LessonReader,
	parameters: {
		layout: 'fullscreen'
	},
	args: {
		lesson: fullLesson,
		onclose: () => {}
	}
} satisfies Meta<LessonReader>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
