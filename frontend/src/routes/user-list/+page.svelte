<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->
<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import { type UserGetAllData, getAllUserData, type UserListResponse } from '$lib/backend/admin';
    import { onMount } from 'svelte';
    import iconEdit from '$lib/assets/icons/edit.svg?raw';
    import { appLink } from '$lib';
    import { booleanToString } from '$lib/index';
    import { calcTotalPages, showUserCount, type PaginationOptions } from '$lib/pagination';

    let userListResponse: null | UserListResponse = $state(null);
    let users: null | UserGetAllData[] = $state(null);

    let pageSizeString = $state('10');
    let pageSizeNumber = $derived(Number(pageSizeString));
    let thisPage = $state(1);

    let paginationOptions: PaginationOptions = $derived({
        page: thisPage,
        size: pageSizeNumber
    });

    onMount(async () => {
        await loadUsers();
    });

    async function loadUsers() {
        userListResponse = await getAllUserData(paginationOptions);
        users = userListResponse.users;
        console.log(userListResponse);
    }
    let prevPageExists = $derived(thisPage > 1);
    let nextPageExists = $derived(
        thisPage < calcTotalPages(showUserCount(userListResponse), pageSizeNumber)
    );

    function prevPage() {
        thisPage -= 1;
        loadUsers();
    }

    function nextPage() {
        thisPage += 1;
        loadUsers();
    }
</script>

<div class="flex w-full justify-center">
    <div class="flex-col">
        <!-- Change the number of users per page -->
        <label>
            {m.numberOfUsers()}
            <select
                class="m-1 w-15 rounded-sm bg-neutral-200 p-1"
                bind:value={pageSizeString}
                onchange={loadUsers}
            >
                <option value="5">5</option>
                <option value="10">10</option>
                <option value="20">20</option>
                <option value="50">50</option>
            </select>
        </label>
    </div>
</div>

{#if users === null}
    <p>{m.loading()}</p>
{:else}
    <div class="100vw flex flex-col items-center justify-center overflow-auto">
        <table class="border-separate border-spacing-2">
            <thead>
                <tr>
                    <th class="text-left">{m.id()}</th>
                    <th class="text-left">{m.email()}</th>
                    <th class="text-left">{m.name()}</th>
                    <th class="text-left">{m.enabled()}</th>
                    <!-- Edit button -->
                    <th class="text-left"></th>
                </tr>
            </thead>
            <tbody>
                {#each users as user (user.id)}
                    <tr>
                        <td class="pr-4 wrap-anywhere">{user.id}</td>
                        <td class="pr-4 wrap-anywhere">{user.email}</td>
                        <td class="pr-4">{user.name}</td>
                        <td class="pr-4">{booleanToString(user.enabled)}</td>
                        <td class="min-w-2 hover:fill-blue-800">
                            <a href={appLink(`/user-manager?id=${user.id}`)} aria-label="edit">
                                <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                                {@html iconEdit}
                            </a>
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
        <div>
            <button
                onclick={prevPage}
                disabled={!prevPageExists}
                class="mx-5"
                class:cursor-pointer={prevPageExists}>Prev</button
            >
            {thisPage}/{calcTotalPages(showUserCount(userListResponse), pageSizeNumber)}
            <button
                onclick={nextPage}
                disabled={!nextPageExists}
                class="mx-5"
                class:cursor-pointer={nextPageExists}>Next</button
            >
        </div>
    </div>
{/if}
