<script lang="ts">
	import NodeViewer from './NodeViewer.svelte';
	import { example_tree } from './example_tree';

	let tree = example_tree;
</script>

<h1>{tree.name}</h1>

<div class="flex h-full flex-col gap-7 overflow-auto">
	{#each tree.roots as root (root)}
		<NodeViewer node={root} />
	{/each}
</div>
