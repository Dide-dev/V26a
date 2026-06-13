<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render the buttons to change pages and page size on a list page with pagination options.

    The `currentPage` and `pageSize` fields should be bound to be updated properly.

    ## Params
    - `currentPage: number` - The bindable current page index.
    - `totalPages: number` - The total number of pages.
    - `handlePageChange: () => void` - The function to run when updating the `currentPage`.
    - `pageSize: string` - The bindable current page size.
    - `handlePageSizeChange: () => void` - The function to run when updating the page size.
    - `children: Snippet` - The contents of the pagination to be rendered based on the page and page size.
-->

<script lang="ts">
    import { PaginationNav } from 'flowbite-svelte';
    import { ChevronLeftOutline, ChevronRightOutline } from 'flowbite-svelte-icons';
    import { m } from '$lib/paraglide/messages.js';
    import type { Snippet } from 'svelte';

    let {
        currentPage = $bindable(),
        totalPages,
        handlePageChange,
        pageSize = $bindable(),
        handlePageSizeChange,
        children
    }: {
        currentPage: number;
        totalPages: number;
        handlePageChange: () => void;
        pageSize: number;
        handlePageSizeChange: () => void;
        children: Snippet;
    } = $props();
</script>

<form class="mb-2 flex w-full flex-row justify-end">
    <label for="page-size" class="flex flex-col justify-center">
        {m.numberPerPage()}
    </label>
    <select
        id="page-size"
        class="input__select ml-2"
        bind:value={pageSize}
        onchange={handlePageSizeChange}
    >
        <option value={5}> 5 </option>
        <option value={10}>10</option>
        <option value={20}>20</option>
        <option value={50}>50</option>
    </select>
</form>

{@render children()}

<PaginationNav
    {currentPage}
    {totalPages}
    onPageChange={(newPage: number) => {
        currentPage = newPage;
        handlePageChange();
    }}
>
    {#snippet prevContent()}
        <span class="sr-only">{m.previousPage()}</span>
        <ChevronLeftOutline class="h-5 w-5" />
    {/snippet}
    {#snippet nextContent()}
        <span class="sr-only">{m.nextPage()}</span>
        <ChevronRightOutline class="h-5 w-5" />
    {/snippet}
</PaginationNav>
