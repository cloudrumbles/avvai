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
		background: linear-gradient(135deg, var(--color-bg) 0%, var(--color-bg-soft) 100%);
		border: 1px solid var(--color-bg-soft);
		border-radius: var(--radius-3);
		padding: var(--space-6) var(--space-7);
	}

	.section-title {
		font-family: var(--font-sans);
		font-size: var(--font-size-4);
		font-weight: 700;
		color: var(--color-accent);
		margin: 0 0 var(--space-4);
		text-align: center;
	}

	.scene-info {
		background: var(--color-bg-soft);
		border-radius: var(--radius-2);
		padding: var(--space-3) var(--space-4);
		margin-bottom: var(--space-6);
		font-family: var(--font-sans);
		font-size: var(--font-size-2-5);
	}

	.scene-setting {
		display: flex;
		gap: var(--space-6);
		flex-wrap: wrap;
		margin-bottom: var(--space-2);
	}

	.scene-setting:last-child {
		margin-bottom: 0;
	}


	.scene-label {
		font-weight: 600;
		color: var(--color-text-subtle);
	}

	.scene-value {
		color: var(--color-text);
	}

	.dialogue-lines {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.dialogue-line {
		display: grid;
		grid-template-columns: auto auto 1fr;
		gap: var(--space-1);
		align-items: baseline;
	}

	.character-name {
		font-family: var(--font-sans);
		font-weight: 700;
		color: var(--color-accent-strong);
		white-space: nowrap;
	}

	.character-separator {
		color: var(--color-text-subtle);
	}

	.dialogue-text {
		color: var(--color-text);
		line-height: var(--line-height-1-8);
	}

	.stage-direction {
		font-family: var(--font-sans);
		font-style: italic;
		color: var(--color-text-subtle);
		text-align: center;
		margin: var(--space-4) 0;
		padding: var(--space-2) var(--space-4);
		background: var(--color-bg-soft);
		border-radius: var(--radius-1-5);
	}

	.narration {
		color: var(--color-text);
		line-height: var(--line-height-1-8);
		margin: 0;
	}

	@media (max-width: 640px) {
		.dialogue-section {
			padding: var(--space-5) var(--space-5);
		}

		.dialogue-line {
			display: block;
		}

		.character-name {
			display: block;
			margin-bottom: var(--space-0-5);
		}

		.character-separator {
			display: none;
		}
	}
</style>