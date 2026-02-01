import type { Meta, StoryObj } from '@storybook/svelte';
import IconButton from '../../lib/components/IconButton.svelte';

const meta = {
	title: 'Components/IconButton',
	component: IconButton,
	args: {
		label: 'Menu',
		disabled: false,
		expanded: false
	}
} satisfies Meta<IconButton>;

export default meta;

type Story = StoryObj<typeof meta>;

const iconMarkup =
	'<svg width="20" height="20" viewBox="0 0 20 20" fill="none" aria-hidden="true">' +
	'<path d="M3 5H17M3 10H17M3 15H17" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>' +
	'</svg>';

const renderIconButton = (args: Story['args']) => ({
	Component: IconButton,
	props: args,
	slots: {
		default: iconMarkup
	}
});

export const Default: Story = {
	render: renderIconButton
};

export const Disabled: Story = {
	args: {
		disabled: true
	},
	render: renderIconButton
};

export const Expanded: Story = {
	args: {
		expanded: true
	},
	render: renderIconButton
};
