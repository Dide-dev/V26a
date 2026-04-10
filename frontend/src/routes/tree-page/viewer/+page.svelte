<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import NodeViewer from '../NodeViewer.svelte';
    import { onMount } from 'svelte';
    import { getTreeWithCards, type TreeWithCards } from '$lib/backend/trees';
    import { page } from '$app/state';
    import { m } from '$lib/paraglide/messages.js';
    import { getUserInfo } from '$lib/backend/users';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';

    //checks if user is logged in or not
    let userInfo = $state(getUserInfo());

    let loading = $state(true);
    let error: Error | null = $state(null);
    let id: number | null = null;
    let tree: TreeWithCards | null = $state(null);

    onMount(async () => {
        const pageId = page.url.searchParams.get('id');

        if (!pageId) {
            error = new Error('No tree id provided');
            return;
        }

        id = Number(pageId);

        if (Number.isNaN(id)) {
            error = new Error('Invalid tree id');
            return;
        }

        tree = await getTreeWithCards(id).then(
            (treeWithCards) => treeWithCards,
            (e) => {
                error = e;
                return null;
            }
        );

        loading = false;
    });

    // navigate user to editor when logged in
    async function gotoEditor() {
        const id = page.url.searchParams.get('id');
        goto(appLink(`/tree-page/editor?id=${id}`));
    }
</script>

{#if error == null && loading}
    <p class="text-green-800">{m.loadingTree()}</p>
{:else if error != null}
    <p class="text-red-700" data-testid="error">{error}</p>
{:else if tree}
    <h1 class="m-auto ml-1.5 text-3xl font-bold">{tree.data.title}</h1>
    <p class="m-auto ml-1.5 pt-1 pb-1 text-sm font-bold text-gray-400">
        {tree.data.description}
    </p>

    <div class="mt-1 mr-1 ml-1 flex grow flex-col gap-7 overflow-auto">
        {#each tree.structure.rootNodes as root (root)}
            <NodeViewer bind:tree parent={root} node={root} editingPage={false} />
        {/each}
    </div>
{/if}

{#if error == null && userInfo !== null}
    <button
        class="edit_button fixed top-16 right-4 z-50 rounded bg-blue-600 px-4 py-2 text-white shadow"
        onclick={() => gotoEditor()}
    >
        Edit
    </button>
{/if}

<!--
    Here we add a few css classes to the body,
    to make sure that the element above here with the `overflow-auto` class can be
    growing in height until the scroll bar is at the bottom of the screen.
-->
<style>
    :global(body) {
        min-height: 100vh;
        display: flex;
        flex-direction: column;
    }
</style>
