import type { Meta, StoryObj } from '@storybook/svelte';
import DictionaryPopup from '../../lib/components/DictionaryPopup.svelte';
import { showDictionary } from '../../lib/stores/dictionary.svelte';

const meta = {
	title: 'Components/DictionaryPopup',
	component: DictionaryPopup,
	parameters: {
		layout: 'fullscreen'
	}
} satisfies Meta<DictionaryPopup>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	play: async () => {
		const mockResponse = {
			ok: true,
			json: async () => ({
				word: 'kurinji',
				definition: 'A mountain landscape associated with early love.',
				examples: ['The mist rests on the hills.']
			})
		} as Response;

		globalThis.fetch = async () => mockResponse;

		showDictionary('kurinji', {
			x: 260,
			y: 200,
			bottom: 220
		});
	}
};
