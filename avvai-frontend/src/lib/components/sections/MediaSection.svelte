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
		margin: 1.5em 0;
	}

	.media-image {
		display: block;
		width: 100%;
		max-width: 100%;
		height: auto;
		border-radius: 8px;
	}

	.media-audio {
		width: 100%;
		max-width: 28rem;
	}

	.media-video {
		width: 100%;
		max-width: 100%;
		border-radius: 8px;
	}

	.media-caption {
		margin-top: 0.5em;
		text-align: center;
		font-size: 0.875em;
		font-style: italic;
		color: #6b5a48;
	}
</style>
