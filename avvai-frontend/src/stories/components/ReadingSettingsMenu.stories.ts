import type { Meta, StoryObj } from '@storybook/svelte';
import ReadingSettingsMenu from '../../lib/components/ReadingSettingsMenu.svelte';
import { DEFAULT_FONT } from '../../lib/config/fonts';

const meta = {
	title: 'Components/ReadingSettingsMenu',
	component: ReadingSettingsMenu,
	args: {
		fontFamily: DEFAULT_FONT,
		fontSize: 18,
		onfontchange: () => {},
		onsizechange: () => {}
	}
} satisfies Meta<ReadingSettingsMenu>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
