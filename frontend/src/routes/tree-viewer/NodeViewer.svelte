<script lang="ts">
	import NodeViewer from './NodeViewer.svelte';
	import type { Node, RootNode } from './example_tree';
	let { node }: { node: Node | RootNode } = $props();
</script>

<div class="flex gap-10">
	{#if 'label' in node}
		<div class="w-32 shrink-0 bg-fuchsia-300">
			<strong>{node.label}</strong>
		</div>
	{:else}
		<div class="h-12 w-xl bg-purple-100 p-1">
			<p>{node.card}</p>
		</div>
	{/if}

	<!-- TODO: some kind of arrows -->

	<div class="flex flex-col gap-5">
		{#each node.children as child_node (child_node)}
			<NodeViewer node={child_node} />
		{/each}
	</div>
</div>
