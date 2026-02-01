import type { Meta, StoryObj } from '@storybook/svelte';
import ExercisesSection from '../../lib/components/sections/ExercisesSection.svelte';
import { exercisesSection } from '../mocks/lesson';

const meta = {
	title: 'Sections/ExercisesSection',
	component: ExercisesSection,
	args: {
		data: exercisesSection
	}
} satisfies Meta<ExercisesSection>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
