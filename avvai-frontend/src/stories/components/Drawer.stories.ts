import type { Meta, StoryObj } from '@storybook/svelte';
import Drawer from '../../lib/components/Drawer.svelte';

const meta = {
	title: 'Components/Drawer',
	component: Drawer,
	parameters: {
		layout: 'fullscreen'
	},
	args: {
		open: true,
		title: 'Notes',
		position: 'bottom',
		onclose: () => {}
	}
} satisfies Meta<Drawer>;

export default meta;

type Story = StoryObj<typeof meta>;

const contentMarkup =
	'<div style="display:flex;flex-direction:column;gap:12px;">' +
	'<p style="margin:0;">A drawer can hold supporting content, settings, or exercises.</p>' +
	'<ul style="margin:0;padding-left:18px;">' +
	'<li>Context notes</li>' +
	'<li>Lesson prompts</li>' +
	'<li>Quick actions</li>' +
	'</ul>' +
	'</div>';

const renderDrawer = (args: Story['args']) => ({
	Component: Drawer,
	props: args,
	slots: {
		default: contentMarkup
	}
});

export const Bottom: Story = {
	args: {
		position: 'bottom'
	},
	render: renderDrawer
};

export const Left: Story = {
	args: {
		position: 'left',
		title: 'Table of Contents'
	},
	render: renderDrawer
};

export const Right: Story = {
	args: {
		position: 'right',
		title: 'Details'
	},
	render: renderDrawer
};
