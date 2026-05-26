<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { htmlFromMarkdown } from '$lib/markdown';
    import type { CardData } from '$lib/api-wrapper/cards';
    let {
        card
    }: {
        card: CardData;
    } = $props();
</script>

<p
    class="h-fit min-h-9 w-xl overflow-auto border-2 border-b-0 border-green-800 bg-green-500 p-1 text-center wrap-anywhere"
>
    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
    {@html htmlFromMarkdown(card.front)}
</p>
<p
    class="h-fit min-h-9 w-xl overflow-auto border-2 border-green-800 bg-green-50 p-1 text-left wrap-anywhere"
>
    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
    {@html htmlFromMarkdown(card.back)}
</p>
