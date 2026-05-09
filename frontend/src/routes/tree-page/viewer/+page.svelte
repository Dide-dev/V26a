<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import NodeViewer from '../NodeViewer.svelte';
    import { onMount } from 'svelte';
    import { getTreeWithCards, type TreeData } from '$lib/backend/trees';
    import { page } from '$app/state';
    import { m } from '$lib/paraglide/messages.js';
    import { getUserInfo } from '$lib/backend/users';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { fillStateStructure, initializeHistory, type TreeHistory } from '../history.svelte';

    // Checks if user is logged in or not.
    let userInfo = $state(getUserInfo());

    let loading = $state(true);
    let userAllowedToEdit = $state(false);
    let id: number | null = null;
    let tree: [TreeData, TreeHistory] | null = $state(null);

    onMount(async () => {
        const pageId = page.url.searchParams.get('id');

        if (!pageId) {
            // TODO: redirect to tree list
            await goto(appLink('/'));
            return;
        }

        id = Number(pageId);

        if (Number.isNaN(id)) {
            // TODO: redirect to tree list
            await goto(appLink('/'));
            return;
        }

        const fetchedTree = await getTreeWithCards(id);
        tree = [
            fetchedTree.metadata.data,
            initializeHistory(
                fetchedTree.metadata.genData.id,
                fillStateStructure(fetchedTree.structure, false),
                fetchedTree.cards
            )
        ];

        loading = false;

        const isOwner = tree && userInfo && fetchedTree.metadata.genData.ownerId === userInfo.sub;
        const isAdmin = userInfo?.accountType === 'admin';

        if (isOwner || isAdmin) {
            userAllowedToEdit = true;
        }
    });
</script>

<div class="flex h-full w-full flex-col items-start overflow-hidden">
    {#if loading}
        <p class="text-green-800">{m.loadingTree()}</p>
    {:else if tree}
        <h1 class="m-auto ml-1.5 text-3xl font-bold">{tree[0].title}</h1>
        <p class="m-auto ml-1.5 pt-1 pb-1 text-sm font-bold text-gray-400">
            {tree[0].description}
        </p>

        <div class="m-1 flex w-full grow flex-col overflow-x-auto pt-6">
            {#each tree[1].currentStructure.rootNodes as root, index (root)}
                <NodeViewer
                    bind:history={tree[1]}
                    location={[index]}
                    editingPage={false}
                    dragState={null}
                    sidebar={null}
                    numPopupsOpen={0}
                />
            {/each}
        </div>
    {/if}
</div>

{#if userAllowedToEdit}
    <button
        data-testid="edit-button"
        class="fixed top-16 right-4 z-50 m-2 cursor-pointer rounded-sm bg-emerald-500 px-4 py-2 shadow"
        onclick={() => goto(appLink(`/tree-page/editor?id=${id}`))}
    >
        {m.edit()}
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
