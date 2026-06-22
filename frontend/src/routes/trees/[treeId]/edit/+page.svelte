<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { onMount } from 'svelte';
    import Lock from './Lock.svelte';
    import TreeEditor from './TreeEditor.svelte';
    import type { TreeId } from '$lib/api-wrapper/trees';

    let { params } = $props();

    let id: TreeId = $derived(Number(params.treeId));

    onMount(async () => {
        if (Number.isNaN(id)) {
            await goto(appLink('/trees'));
            return;
        }
    });
</script>

{#if id !== null}
    <Lock treeId={id}>
        <TreeEditor {id} />
    </Lock>
{/if}
