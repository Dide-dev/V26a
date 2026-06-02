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
        outOfFocus
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
        focus = false;
    }

    function blur() {
        focus = true;
    }

    let focus: boolean | null = $state(null)

    let openColor = $derived(`var(--color-${baseColor}-100)`);
    let openHoverColor = $derived(`var(--color-${baseColor}-50)`);
    let openBorderColor = $derived(`var(--color-${baseColor}-dim-350)`);
    let hoverColor = $derived(`var(--color-${baseColor}-500)`);
    let blurColor = $derived(`var(--color-${baseColor}-dim-50)`);
    let blurHoverColor = $derived(`var(--color-${baseColor}-dim-150)`);
    let blurBorderColor = $derived(`var(--color-${baseColor}-dim-250)`);

    let closedColor = $derived(`var(--color-${baseColor}-${Math.min(900, 500 + 100 * distanceFromRevealed)})`);
    let borderColor = $derived(`var(--color-${baseColor}-${Math.min(950, 700 + 100 * distanceFromRevealed)})`);
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
        class="card min-h-card-height-sm w-card-width-sm text-left p-1 cursor-pointer text-lg text-neutral-600"
        onclick={refocus}
    >
        ...
    </button>
{/snippet}

{#snippet revealedCard(card: CardData)}
    <button
        type="button"
        style="--background-color: {openColor}; --hover-color: {openHoverColor}; --border-color: {openBorderColor}"
        class="card min-h-card-height-md w-card-width-md overflow-x-auto p-1 wrap-anywhere cursor-pointer"
        onclick={blur}
    >
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html markdownToFinalHtml(card.back)}
    </button>
{/snippet}

{#snippet openableCard(card: CardData)}
    <button
        style="--background-color: {closedColor}; --hover-color: {hoverColor}; --border-color: {borderColor}"
        class="card min-h-card-height-md w-card-width-md text-lg italic text-neutral-200 hover:text-neutral-100 cursor-pointer overflow-x-auto p-1 text-center wrap-anywhere"
        onclick={makeRevealed}
    >
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html markdownToFinalHtml(card.front)}
    </button>
{/snippet}

{#snippet closedCard()}
    <p
        style="--background-color: {closedColor}; --hover-color: {closedColor}; --border-color: {borderColor}"
       class={[distanceFromRevealed == 2 ? 'w-card-width-sm' : 'w-card-width-xs', "card min-h-card-height-sm p-1"]}
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

            border-color: var(--border-color);

            &:hover {
                background-color: var(--hover-color);
            }
        }
    }
</style>
