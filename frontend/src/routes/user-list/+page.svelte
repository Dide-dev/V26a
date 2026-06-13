<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->
<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import { getAllUserData, type UserListResponse } from '$lib/api-wrapper/admin';
    import { onMount } from 'svelte';
    import { calcTotalPages, type PaginationOptions } from '$lib/pagination';
    import ListPagination from '$lib/lists/ListPagination.svelte';
    import UserListEntry from '$lib/lists/UserListEntry.svelte';

    let userListResponse = $state(null as null | UserListResponse);

    let pageSize = $state(10);
    let currentPage = $state(1);
    let totalPages = $derived(calcTotalPages(userListResponse?.totalUsers ?? 0, pageSize));

    let paginationOptions: PaginationOptions = $derived({
        page: currentPage,
        size: pageSize
    });

    onMount(async () => {
        await loadUsers();
    });

    async function loadUsers() {
        userListResponse = await getAllUserData(paginationOptions);
    }

    function handlePageChange() {
        loadUsers();
    }

    function handlePageSizeChange() {
        currentPage = 1;
        loadUsers();
    }
</script>

<ListPagination
    bind:currentPage
    {totalPages}
    {handlePageChange}
    bind:pageSize
    {handlePageSizeChange}
>
    {#snippet title()}
        <h1 class="page__title">{m.userList()}</h1>
    {/snippet}

    {#if userListResponse?.users === null}
        <p>{m.loading()}</p>
    {:else}
        <div class="list__container">
            {#each userListResponse?.users as user (user)}
                <UserListEntry {user} />
            {/each}
        </div>
    {/if}
</ListPagination>
