import type { Meta, StoryObj } from '@storybook/svelte';
import PrimitivesShowcase from './PrimitivesShowcase.svelte';

const meta = {
	title: 'Design System/Primitives',
	component: PrimitivesShowcase
} satisfies Meta<PrimitivesShowcase>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Buttons: Story = {
	args: { section: 'buttons' }
};

export const Inputs: Story = {
	args: { section: 'inputs' }
};

export const CardsAndBadges: Story = {
	args: { section: 'cards' }
};

export const IconAndInteractive: Story = {
	args: { section: 'interactive' }
};
