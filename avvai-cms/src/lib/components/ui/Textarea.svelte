<script lang="ts">
	import Input from './Input.svelte';

	interface Props {
		label?: string;
		value?: string;
		placeholder?: string;
		error?: string;
		disabled?: boolean;
		rows?: number;
		class?: string;
		oninput?: (e: Event & { currentTarget: HTMLInputElement | HTMLTextAreaElement }) => void;
	}

	let {
		label,
		value = $bindable(''),
		placeholder = '',
		error,
		disabled = false,
		rows = 4,
		class: className = '',
		oninput
	}: Props = $props();
</script>

<Input
	{label}
	bind:value
	{placeholder}
	{error}
	{disabled}
	{rows}
	multiline
	class={className}
	{oninput}
/>
