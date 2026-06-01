<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { markdownToFinalHtml } from '$lib/rich-text/markdown';
    import type { CardData } from '$lib/api-wrapper/cards';

    let {
        card,
        baseColor,
        distanceFromRevealed = $bindable(),
        outOfFocus = $bindable()
    }: {
        card: CardData;
        baseColor: string;
        distanceFromRevealed: number;
        outOfFocus: boolean;
    } = $props();

    function makeRevealed() {
        distanceFromRevealed = 0;
    }

    function refocus() {
        console.log("focus");
        outOfFocus = false;
    }

    function blur() {
        console.log("blur");
        outOfFocus = true;
    }

    // --color-tree-200 --color-tree-500 --color-tree-600 --color-tree-650 --color-tree-700 --color-tree-750 --color-tree-800

    let openColor = $derived(`var(--color-${baseColor}-200)`);
    let hoverColor = $derived(`var(--color-${baseColor}-500)`);

    let closedColor = $derived(`var(--color-${baseColor}-${Math.min(800, 550 + 50 * distanceFromRevealed)})`);
    let borderColor = $derived(`var(--color-${baseColor}-${Math.min(900, 650 + 50 * distanceFromRevealed)})`);
</script>
{#if outOfFocus}
    {@render outOfFocusCard()}
{:else if distanceFromRevealed === 0}
    {@render revealedCard(card)}
{:else if distanceFromRevealed === 1}
    {@render openableCard(card)}
{:else}
    {@render closedCard()}
{/if}

{#snippet outOfFocusCard()}
    <button
        type="button"
        style="--background-color: {openColor}; --hover-color: {openColor}"
        class="card min-h-card-height-sm w-card-width-sm text-left p-1 cursor-pointer"
        onclick={refocus}
    >
        ...
    </button>
{/snippet}

{#snippet revealedCard(card: CardData)}
    <button
        type="button"
        style="--background-color: {openColor}; --hover-color: {openColor}"
        class="card min-h-card-height-md w-card-width-md overflow-x-auto border-green-800 p-1 wrap-anywhere cursor-pointer"
        onclick={blur}
    >
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html markdownToFinalHtml(card.back)}
    </button>
{/snippet}

{#snippet openableCard(card: CardData)}
    <button
        style="--background-color: {closedColor}; --hover-color: {hoverColor}"
        class="card min-h-card-height-md w-card-width-md text-lg italic text-neutral-200 hover:text-neutral-100 cursor-pointer overflow-x-auto border-green-800 p-1 text-center wrap-anywhere"
        onclick={makeRevealed}
    >
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html markdownToFinalHtml(card.front)}
    </button>
{/snippet}

{#snippet closedCard()}
    <p
        style="--background-color: {closedColor}; --hover-color: {closedColor}"
       class={[distanceFromRevealed == 2 ? 'w-card-width-sm' : 'w-card-width-xs', "card min-h-card-height-sm p-1"]}
    ></p>
{/snippet}

<style>
    @layer components {
        .card {
            height: fit-content;

            border-radius: var(--radius-md);
            border-width: 2px;

            background-color: var(--background-color);

            &:hover {
                background-color: var(--hover-color);
            }
        }
    }
</style>
