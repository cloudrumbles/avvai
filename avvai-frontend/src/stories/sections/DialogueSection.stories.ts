import type { Meta, StoryObj } from '@storybook/svelte';
import DialogueSection from '../../lib/components/sections/DialogueSection.svelte';
import { dialogueSection } from '../mocks/lesson';

const meta = {
	title: 'Sections/DialogueSection',
	component: DialogueSection,
	args: {
		data: dialogueSection
	}
} satisfies Meta<DialogueSection>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
