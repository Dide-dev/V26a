<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { followIndex, followIndexNode, setRevealed, type TreeHistory } from './history.svelte';
    import { htmlFromMarkdown } from '$lib/markdown';
    let {
        history,
        index,
        editingPage
    }: {
        history: TreeHistory;
        index: number[];
        editingPage: boolean;
    } = $props();

    let node = $derived(followIndexNode(history.currentStructure, index));
    let card = $derived(history.currentCards[node.cardId]);
    let visible = $derived(node.revealed);

    let parentIsVisible = $derived.by(() => {
        let parent = followIndex(history.currentStructure, index.slice(0, -1));
        if ('revealed' in parent) {
            return parent.revealed;
        } else {
            return true;
        }
    });

    function setVisible() {
        setRevealed(history, index, true);
    }
    function toggleVisible() {
        setRevealed(history, index, !visible);
    }
</script>

{#if editingPage}
    <!-- Cards layout in the editor -->
    {#if visible}
        <button
            class="h-fit min-h-9 w-xl cursor-pointer overflow-auto border-2 border-green-800 bg-green-50 p-1 text-left wrap-anywhere"
            onclick={toggleVisible}
        >
            <!-- eslint-disable-next-line svelte/no-at-html-tags -->
            {@html htmlFromMarkdown(card.back)}
        </button>
    {:else}
        <button
            class="h-fit min-h-9 w-xl cursor-pointer overflow-auto border-2 border-green-800 bg-green-500 p-1 text-center wrap-anywhere"
            onclick={toggleVisible}
        >
            <!-- eslint-disable-next-line svelte/no-at-html-tags -->
            {@html htmlFromMarkdown(card.front)}
        </button>
    {/if}
{:else}
    <!-- Cards layout in the viewer -->
    {#if visible}
        <p
            class="h-fit min-h-9 w-xl overflow-auto border-2 border-green-800 bg-green-50 p-1 wrap-anywhere"
        >
            <!-- eslint-disable-next-line svelte/no-at-html-tags -->
            {@html htmlFromMarkdown(card.back)}
        </p>
    {:else if parentIsVisible}
        <button
            class="h-fit min-h-9 w-fit max-w-xl min-w-50 cursor-pointer overflow-auto border-2 border-green-800 bg-green-500 p-1 text-center wrap-anywhere"
            onclick={setVisible}
        >
            <!-- eslint-disable-next-line svelte/no-at-html-tags -->
            {@html htmlFromMarkdown(card.front)}
        </button>
    {:else}
        <p class="tree-node h-fit min-h-9 w-4 border-2 border-green-800 bg-green-800 p-1"><br /></p>
    {/if}
{/if}
