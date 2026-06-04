<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render a card that can be opened or closed.

    ## Params
    - `card: CardData` - The contents of the card.
    - `color: BranchColor` - The name of the color.
    - `distanceFromRevealed: number` - The distance from the closest revealed card.
                                       If `0`, the card is revealed.
    - `outOfFocus: boolean` - Whether all children have been opened,
                              and the card is thus *out of focus*.
-->

<script lang="ts">
    import { markdownToFinalHtml } from '$lib/rich-text/markdown';
    import type { CardData } from '$lib/api-wrapper/cards';
    import type { BranchColor } from '$lib/treePage';

    let {
        card,
        color,
        distanceFromRevealed = $bindable(),
        outOfFocus
    }: {
        card: CardData;
        color: BranchColor;
        distanceFromRevealed: number;
        outOfFocus: boolean;
    } = $props();

    function makeRevealed() {
        distanceFromRevealed = 0;
    }

    function refocus() {
        focus = false;
    }

    function blur() {
        focus = true;
    }

    let focus: boolean | null = $state(null);

    let openColor = $derived(`var(--color-${color}-100)`);
    let openHoverColor = $derived(`var(--color-${color}-50)`);
    let openBorderColor = $derived(`var(--color-${color}-dim-350)`);
    let hoverColor = $derived(`var(--color-${color}-500)`);
    let blurColor = $derived(`var(--color-${color}-dim-50)`);
    let blurHoverColor = $derived(`var(--color-${color}-dim-150)`);
    let blurBorderColor = $derived(`var(--color-${color}-dim-250)`);

    let closedColor = $derived(
        `var(--color-${color}-${Math.min(900, 500 + 100 * distanceFromRevealed)})`
    );
    // --color-cyan-950 --color-tree-950 --color-yellow-950 --color-red-950 --color-pink-950 --color-violet-950
    let borderColor = $derived(
        `var(--color-${color}-${Math.min(950, 700 + 100 * distanceFromRevealed)})`
    );
</script>

{#if focus ?? outOfFocus}
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
        style="--background-color: {blurColor}; --hover-color: {blurHoverColor}; --border-color: {blurBorderColor}"
        class="card min-h-card-height-sm w-card-width-sm cursor-pointer p-1 text-left text-lg text-neutral-600"
        onclick={refocus}
    >
        ...
    </button>
{/snippet}

{#snippet revealedCard(card: CardData)}
    <button
        type="button"
        style="--background-color: {openColor}; --hover-color: {openHoverColor}; --border-color: {openBorderColor}"
        class="card min-h-card-height-md w-card-width-lg cursor-pointer overflow-x-auto p-1 wrap-anywhere"
        onclick={blur}
    >
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html markdownToFinalHtml(card.back)}
    </button>
{/snippet}

{#snippet openableCard(card: CardData)}
    <button
        style="--background-color: {closedColor}; --hover-color: {hoverColor}; --border-color: {borderColor}"
        class="card min-h-card-height-md w-card-width-lg cursor-pointer overflow-x-auto p-1 text-center text-lg wrap-anywhere text-neutral-200 italic hover:text-neutral-100"
        onclick={makeRevealed}
    >
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html markdownToFinalHtml(card.front)}
    </button>
{/snippet}

{#snippet closedCard()}
    <p
        style="--background-color: color-mix(in oklch, {closedColor} 40%, oklch(from {closedColor} l 0 h)); --hover-color: var(--background-color); --border-color: {borderColor}"
        class={[
            distanceFromRevealed == 2 ? 'w-card-width-sm' : 'w-card-width-xs',
            'card min-h-card-height-sm p-1'
        ]}
    ></p>
{/snippet}

<style>
    @import './../../lib/theme.css';

    @layer components {
        .card {
            height: fit-content;

            border-radius: var(--radius-md);
            border-width: 2px;

            background-color: var(--background-color);
            transition: background-color 200ms ease-in-out;

            border-color: var(--border-color);

            &:hover {
                background-color: var(--hover-color);
            }
        }
    }
</style>
