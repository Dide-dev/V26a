<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { htmlFromMarkdown } from '$lib/markdown';
    import type { CardGetData } from '$lib/backend/cards';
    let {
        card,
        revealed = $bindable(),
        parentRevealed
    }: {
        card: CardGetData;
        revealed: boolean;
        parentRevealed: boolean;
    } = $props();

    function makeRevealed() {
        // setRevealed(history, location, true);
        revealed = true;
    }
</script>

{#if revealed}
    <p
        class="h-fit min-h-9 w-xl overflow-auto border-2 border-green-800 bg-green-50 p-1 wrap-anywhere"
    >
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html htmlFromMarkdown(card.back)}
    </p>
{:else if parentRevealed}
    <button
        class="h-fit min-h-9 w-fit max-w-xl min-w-50 cursor-pointer overflow-auto border-2 border-green-800 bg-green-500 p-1 text-center wrap-anywhere"
        onclick={makeRevealed}
    >
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html htmlFromMarkdown(card.front)}
    </button>
{:else}
    <p class="tree-node h-fit min-h-9 w-4 border-2 border-green-800 bg-green-800 p-1"><br /></p>
{/if}
