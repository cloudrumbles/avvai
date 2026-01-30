<script lang="ts">
	import Button from '$lib/components/ui/Button.svelte';
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import ExerciseGroupEditor from './ExerciseGroupEditor.svelte';
	import type { ExercisesSection } from '$lib/types/lesson';

	interface Props {
		section: ExercisesSection;
	}

	let { section = $bindable() }: Props = $props();

	function addGroup() {
		section.exercise_groups = [
			...section.exercise_groups,
			{
				group_type: 'multiple_choice',
				instructions: '',
				exercises: []
			}
		];
	}

	function removeGroup(index: number) {
		const confirm = window.confirm("Are you sure you want to delete this exercise group?");
		if (confirm) {
			section.exercise_groups = section.exercise_groups.filter((_, i) => i !== index);
		}
	}
</script>

<div class="exercises-editor">
	<TextInput
		label="Section Title (optional)"
		value={section.title ?? ''}
		placeholder="e.g. Practice Questions"
		oninput={(e) => {
			section.title = e.currentTarget.value || undefined;
		}}
	/>

	<div class="groups-list">
		{#each section.exercise_groups as group, index}
			<ExerciseGroupEditor 
				bind:group={section.exercise_groups[index]} 
				index={index} 
				onDelete={() => removeGroup(index)}
			/>
		{/each}

		<div class="add-group-action">
			<Button variant="ghost" onclick={addGroup}>
				+ Add Exercise Group
			</Button>
		</div>
	</div>
</div>

<style>
	.exercises-editor {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.groups-list {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.add-group-action {
		display: flex;
		justify-content: center;
		padding: 1rem;
		border: 2px dashed rgba(92, 74, 61, 0.1);
		border-radius: 8px;
	}
</style>