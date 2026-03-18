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
