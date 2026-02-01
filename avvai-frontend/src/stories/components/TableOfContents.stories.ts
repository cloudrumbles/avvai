import type { Meta, StoryObj } from '@storybook/svelte';
import TableOfContents from '../../lib/components/TableOfContents.svelte';
import { contentSections } from '../mocks/lesson';

const meta = {
	title: 'Components/TableOfContents',
	component: TableOfContents,
	args: {
		sections: contentSections,
		activeSectionIndex: 1,
		progressPercent: 38,
		onSectionClick: () => {}
	}
} satisfies Meta<TableOfContents>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const NearComplete: Story = {
	args: {
		activeSectionIndex: 4,
		progressPercent: 82
	}
};
