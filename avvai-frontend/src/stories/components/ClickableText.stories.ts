import type { Meta, StoryObj } from '@storybook/svelte';
import ClickableText from '../../lib/components/ClickableText.svelte';

const meta = {
	title: 'Components/ClickableText',
	component: ClickableText,
	args: {
		text: 'The hill breeze arrives at dusk with the scent of rain.',
		onwordclick: () => {}
	}
} satisfies Meta<ClickableText>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
