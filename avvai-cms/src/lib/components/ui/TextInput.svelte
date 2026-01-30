<script lang="ts">
	import Input from './Input.svelte';

	interface Props {
		label?: string;
		value?: string;
		type?: string;
		placeholder?: string;
		error?: string;
		disabled?: boolean;
		class?: string;
		oninput?: (e: Event & { currentTarget: HTMLInputElement | HTMLTextAreaElement }) => void;
	}

	let {
		label,
		value = $bindable(''),
		type = 'text',
		placeholder = '',
		error,
		disabled = false,
		class: className = '',
		oninput
	}: Props = $props();
</script>

<Input
	{label}
	bind:value
	{type}
	{placeholder}
	{error}
	{disabled}
	class={className}
	{oninput}
/>
