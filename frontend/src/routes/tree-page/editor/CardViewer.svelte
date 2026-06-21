<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { markdownToFinalHtml } from '$lib/rich-text/markdown';
    import type { CardData } from '$lib/api-wrapper/cards';
    import type { BranchColor } from '$lib/treePage';

    let {
        card,
        color,
        isBeingEdited
    }: {
        card: CardData;
        color: BranchColor;
        isBeingEdited: boolean;
    } = $props();

    let frontColor = $derived(`var(--color-${color}-600)`);
    let backColor = $derived(`var(--color-${color}-100)`);
    let borderColor = $derived(
        isBeingEdited ? 'var(--color-yellow-400)' : `var(--color-${color}-800)`
    );
</script>

<p
    style="background-color: {frontColor}; border-color: {borderColor};}"
    class={[
        isBeingEdited && 'border-4',
        'h-fit min-h-9 w-xl overflow-x-auto overflow-y-hidden border-2 border-b-0 p-1 text-center wrap-anywhere'
    ]}
>
    {card.front}
</p>
<p
    style="background-color: {backColor}; border-color: {borderColor};"
    class={[
        isBeingEdited && 'border-4',
        'h-fit min-h-9 w-xl overflow-x-auto overflow-y-hidden border-2 border-t-2 p-1 text-left wrap-anywhere'
    ]}
>
    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
    {@html markdownToFinalHtml(card.back)}
</p>
