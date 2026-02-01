import type { Meta, StoryObj } from '@storybook/svelte';
import MediaSection from '../../lib/components/sections/MediaSection.svelte';

const meta = {
	title: 'Sections/MediaSection',
	component: MediaSection
} satisfies Meta<MediaSection>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Image: Story = {
	args: {
		data: {
			media_type: 'image',
			url: 'https://placehold.co/960x540?text=Lesson+Image',
			caption: 'An image used in a lesson section.'
		}
	}
};

export const Audio: Story = {
	args: {
		data: {
			media_type: 'audio',
			url: 'https://www.w3schools.com/html/horse.mp3',
			caption: 'Short audio clip.'
		}
	}
};

export const Video: Story = {
	args: {
		data: {
			media_type: 'video',
			url: 'https://www.w3schools.com/html/mov_bbb.mp4',
			caption: 'Short video clip.'
		}
	}
};
