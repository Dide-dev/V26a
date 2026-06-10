<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render the buttons to change pages on a list page with pagination options.

    ## Params
    - `currentPage: number` - The current page index.
    - `totalPages: number` - The total number of pages.
    - `handlePageChange: (currentPage: number) => void` - The function to update the `currentPage`.
-->

<script lang="ts">
    import { PaginationNav } from 'flowbite-svelte';
    import { ChevronLeftOutline, ChevronRightOutline } from 'flowbite-svelte-icons';
    import { m } from '$lib/paraglide/messages.js';
    import type { Snippet } from 'svelte';

    let {
        currentPage,
        totalPages,
        handlePageChange,
        pageSize = $bindable(),
        handlePageSizeChange,
        children
    }: {
        currentPage: number;
        totalPages: number;
        handlePageChange: (currentPage: number) => void;
        pageSize: string;
        handlePageSizeChange: () => void;
        children: Snippet;
    } = $props();
</script>

<label>
    {m.numberPerPage({ n: m.treesUncapitalized() })}
    <select
        class="m-1 w-15 rounded-sm bg-neutral-200 p-1"
        bind:value={pageSize}
        onchange={handlePageSizeChange}
    >
        <option value="5">5</option>
        <option value="10">10</option>
        <option value="20">20</option>
        <option value="50">50</option>
    </select>
</label>

{@render children()}

<PaginationNav {currentPage} {totalPages} onPageChange={handlePageChange}>
    {#snippet prevContent()}
        <span class="sr-only">{m.previousPage()}</span>
        <ChevronLeftOutline class="h-5 w-5" />
    {/snippet}
    {#snippet nextContent()}
        <span class="sr-only">{m.nextPage()}</span>
        <ChevronRightOutline class="h-5 w-5" />
    {/snippet}
</PaginationNav>
