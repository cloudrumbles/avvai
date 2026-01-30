<script lang="ts">
	interface Props {
		size?: number;
		color?: string;
	}

	let { size = 24, color = 'currentColor' }: Props = $props();
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill="none">
	<path d="M12 3C9.5 3 7 4.5 7 7C7 10 5 11 5 13H19C19 11 17 10 17 7C17 4.5 14.5 3 12 3Z" stroke={color} stroke-width="2" stroke-linejoin="round"/>
	<path d="M10 17C10 18.1 10.9 19 12 19C13.1 19 14 18.1 14 17" stroke={color} stroke-width="2" stroke-linecap="round"/>
	<path d="M5 13V15H19V13" stroke={color} stroke-width="2" stroke-linecap="round"/>
</svg>
