<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render a both sides of a card for printing purposes.

    ## Params
    - `card: CardData` - The contents of the card.
    - `color: BranchColor` - The name of the color.
-->

<script lang="ts">
    import { markdownToFinalHtml } from '$lib/rich-text/markdown';
    import type { CardData } from '$lib/api-wrapper/cards';
    import type { BranchColor } from '$lib/treePage';
    import type { PostItSize } from './postItSizes';

    let {
        card,
        color,
        cardSize
    }: {
        card: CardData;
        color: BranchColor;
        cardSize: PostItSize;
    } = $props();

    let frontColor = $derived(`var(--color-${color}-300)`);
    let backColor = $derived(`var(--color-${color}-100)`);
    let cardBorderColor = $derived(`var(--color-${color}-dim-350)`);
</script>

<div class="flex flex-col">
    <p
        style="--background-color: {frontColor}; --border-color: {cardBorderColor}; width: {cardSize.width}mm;"
        class="min-h-card-height-md card border-b-0 rounded-b-none overflow-x-auto overflow-y-hidden p-1 text-center wrap-anywhere"
    >
        {card.front}
    </p>
    <div style="min-height: {cardSize.height}mm;">
        <p
            style="--background-color: {backColor}; --border-color: {cardBorderColor}; width: {cardSize.width}mm;"
            class="card rounded-t-none overflow-y-hidden p-1 text-left wrap-anywhere"
        >
            <!-- eslint-disable-next-line svelte/no-at-html-tags -->
            {@html markdownToFinalHtml(card.back)}
        </p>
    </div>
</div>

<style>
    @import '../../../../lib/theme.css';

    @layer components {
        .card {
            height: fit-content;

            border-radius: var(--radius-md);
            border-width: 2px;

            background-color: var(--background-color);

            border-color: var(--border-color);
        }
    }
</style>
