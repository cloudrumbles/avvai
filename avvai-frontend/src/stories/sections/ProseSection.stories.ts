import type { Meta, StoryObj } from '@storybook/svelte';
import ProseSection from '../../lib/components/sections/ProseSection.svelte';
import { proseSection } from '../mocks/lesson';

const meta = {
	title: 'Sections/ProseSection',
	component: ProseSection,
	args: {
		data: proseSection
	}
} satisfies Meta<ProseSection>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
