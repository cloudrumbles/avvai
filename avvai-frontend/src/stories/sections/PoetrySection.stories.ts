import type { Meta, StoryObj } from '@storybook/svelte';
import PoetrySection from '../../lib/components/sections/PoetrySection.svelte';
import { poetrySection } from '../mocks/lesson';

const meta = {
	title: 'Sections/PoetrySection',
	component: PoetrySection,
	args: {
		data: poetrySection
	}
} satisfies Meta<PoetrySection>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
