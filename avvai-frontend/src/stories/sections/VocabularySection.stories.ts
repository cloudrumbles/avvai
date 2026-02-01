import type { Meta, StoryObj } from '@storybook/svelte';
import VocabularySection from '../../lib/components/sections/VocabularySection.svelte';
import { vocabularySection } from '../mocks/lesson';

const meta = {
	title: 'Sections/VocabularySection',
	component: VocabularySection,
	args: {
		data: vocabularySection
	}
} satisfies Meta<VocabularySection>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
