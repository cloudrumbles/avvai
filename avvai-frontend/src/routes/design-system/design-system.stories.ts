import type { Meta, StoryObj } from '@storybook/svelte';
import DesignSystemPage from './+page.svelte';

const meta = {
	title: 'Design System/Overview',
	component: DesignSystemPage
} satisfies Meta<DesignSystemPage>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Overview: Story = {};
