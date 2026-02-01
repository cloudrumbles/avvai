<script lang="ts">
	interface MediaData {
		media_type: 'image' | 'audio' | 'video';
		url: string;
		caption?: string;
	}

	interface Props {
		data: MediaData;
	}

	let { data }: Props = $props();
</script>

<div class="media-section">
	{#if data.media_type === 'image'}
		<img
			src={data.url}
			alt={data.caption ?? ''}
			class="media-image"
		/>
	{:else if data.media_type === 'audio'}
		<audio controls src={data.url} class="media-audio">
			Your browser does not support the audio element.
		</audio>
	{:else if data.media_type === 'video'}
		<!-- svelte-ignore a11y_media_has_caption -->
		<video controls src={data.url} class="media-video">
			Your browser does not support the video element.
		</video>
	{/if}

	{#if data.caption}
		<p class="media-caption">{data.caption}</p>
	{/if}
</div>

<style>
	.media-section {
		width: 100%;
		margin: var(--space-6) 0;
	}

	.media-image {
		display: block;
		width: 100%;
		max-width: 100%;
		height: auto;
		border-radius: var(--radius-2);
	}

	.media-audio {
		width: 100%;
		max-width: 28rem;
	}

	.media-video {
		width: 100%;
		max-width: 100%;
		border-radius: var(--radius-2);
	}

	.media-caption {
		margin-top: var(--space-2);
		text-align: center;
		font-size: var(--font-size-2);
		font-style: italic;
		color: var(--color-text-subtle);
	}
</style>