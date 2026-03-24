<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { page } from '$app/state';
    import { onMount } from 'svelte';

    let treeId: string | null = null;

    onMount(() => {
        treeId = page.url.searchParams.get('id');
    });
</script>

<div>
    {treeId}
</div>
