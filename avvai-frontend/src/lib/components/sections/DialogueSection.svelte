<script lang="ts">
	import ClickableText from 'avvai-frontend/components/ClickableText';

	interface SceneInfo {
		location?: string;
		time?: string;
		characters?: string[];
	}

	interface DialogueLine {
		character?: string;
		text: string;
		direction?: boolean;
	}

	interface DialogueSectionData {
		title?: string;
		scene?: SceneInfo;
		lines: DialogueLine[];
	}

	interface Props {
		data: DialogueSectionData;
	}

	let { data }: Props = $props();
</script>

<section class="dialogue-section">
	{#if data.title}
		<h2 class="section-title">{data.title}</h2>
	{/if}

	{#if data.scene}
		<div class="scene-info">
			{#if data.scene.location || data.scene.time}
				<div class="scene-setting">
					{#if data.scene.location}
						<span class="scene-label">இடம்:</span>
						<span class="scene-value">{data.scene.location}</span>
					{/if}
					{#if data.scene.time}
						<span class="scene-label">நேரம்:</span>
						<span class="scene-value">{data.scene.time}</span>
					{/if}
				</div>
			{/if}
			{#if data.scene.characters && data.scene.characters.length > 0}
				<div class="scene-characters">
					<span class="scene-label">பங்கேற்போர்:</span>
					<span class="scene-value">{data.scene.characters.join(', ')}</span>
				</div>
			{/if}
		</div>
	{/if}

	<div class="dialogue-lines">
		{#each data.lines as line, i (i)}
			{#if line.direction}
				<p class="stage-direction">{line.text}</p>
			{:else if line.character}
				<div class="dialogue-line">
					<span class="character-name">{line.character}</span>
					<span class="character-separator">:</span>
					<span class="dialogue-text">
						<ClickableText text={line.text} />
					</span>
				</div>
			{:else}
				<p class="narration">
					<ClickableText text={line.text} />
				</p>
			{/if}
		{/each}
	</div>
</section>

<style>
	.dialogue-section {
		background: linear-gradient(135deg, var(--cream) 0%, rgba(139, 26, 26, 0.03) 100%);
		border: 1px solid var(--cream-mid);
		border-radius: 12px;
		padding: 24px 28px;
	}

	.section-title {
		font-family: 'Catamaran', sans-serif;
		font-size: 1.1em;
		font-weight: 700;
		color: var(--red);
		margin: 0 0 1em;
		text-align: center;
	}

	.scene-info {
		background: var(--cream-mid);
		border-radius: 8px;
		padding: 12px 16px;
		margin-bottom: 1.5em;
		font-family: 'Catamaran', sans-serif;
		font-size: 0.9em;
	}

	.scene-setting {
		display: flex;
		gap: 1.5em;
		flex-wrap: wrap;
		margin-bottom: 0.5em;
	}

	.scene-setting:last-child {
		margin-bottom: 0;
	}

	.scene-characters {
		/* Characters line */
	}

	.scene-label {
		font-weight: 600;
		color: var(--stone);
	}

	.scene-value {
		color: var(--ink);
	}

	.dialogue-lines {
		display: flex;
		flex-direction: column;
		gap: 1em;
	}

	.dialogue-line {
		display: grid;
		grid-template-columns: auto auto 1fr;
		gap: 0.3em;
		align-items: baseline;
	}

	.character-name {
		font-family: 'Catamaran', sans-serif;
		font-weight: 700;
		color: var(--red-deep);
		white-space: nowrap;
	}

	.character-separator {
		color: var(--stone);
	}

	.dialogue-text {
		color: var(--ink);
		line-height: 1.8;
	}

	.stage-direction {
		font-family: 'Catamaran', sans-serif;
		font-style: italic;
		color: var(--stone);
		text-align: center;
		margin: 1em 0;
		padding: 0.5em 1em;
		background: var(--cream-mid);
		border-radius: 6px;
	}

	.narration {
		color: var(--ink);
		line-height: 1.8;
		margin: 0;
	}

	@media (max-width: 640px) {
		.dialogue-section {
			padding: 20px 20px;
		}

		.dialogue-line {
			display: block;
		}

		.character-name {
			display: block;
			margin-bottom: 0.2em;
		}

		.character-separator {
			display: none;
		}
	}
</style>
