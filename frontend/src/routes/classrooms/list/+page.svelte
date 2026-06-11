<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import { type ClassroomListResponse } from '$lib/api-wrapper/classrooms';
    import { getAllOwnedClassrooms } from '$lib/api-wrapper/users';
    import { appLink } from '$lib';
    import { onMount } from 'svelte';
    import { calcTotalPages, type PaginationOptions } from '$lib/pagination';
    import { getUserInfo } from '$lib/api-wrapper/users';
    import { redirectLogin } from '$lib/api-wrapper';
    import { page } from '$app/state';

    // This cannot be done with the {#await ...} in svelte,
    // because then updating it browser-side for classroom deletion is not possible.
    let classroomListResponse: ClassroomListResponse = $state({
        totalClassrooms: 0,
        classrooms: []
    });

    let pageSizeString = $state('10');
    let pageSizeNumber = $derived(Number(pageSizeString));
    let thisPage = $state(1);

    onMount(async () => {
        await loadClassrooms();
    });

    async function loadClassrooms() {
        let userInfo = getUserInfo();

        let paginationOptions: PaginationOptions = {
            page: thisPage,
            size: pageSizeNumber
        };
        if (userInfo !== null) {
            classroomListResponse = await getAllOwnedClassrooms(userInfo.sub, paginationOptions);
        } else {
            await redirectLogin(page.url);
        }
    }

    let prevPageExists = $derived(thisPage > 1);
    let nextPageExists = $derived.by(() => {
        // For some reason svelte does not understand the type of classroomListResponse so we have to manually cast it.
        const total = (classroomListResponse as ClassroomListResponse | null)?.totalClassrooms ?? 0;
        return thisPage < calcTotalPages(total, pageSizeNumber);
    });

    function changePageSize() {
        thisPage = 1;
        loadClassrooms();
    }

    function prevPage() {
        thisPage -= 1;
        loadClassrooms();
    }

    function nextPage() {
        thisPage += 1;
        loadClassrooms();
    }
</script>

<div class="flex w-full justify-center">
    <div class="flex-col">
        <!-- Change the number of classrooms per page -->
        <label>
            {m.numberPerPage({ n: m.classroomsUncapitalized() })}
            <select
                class="m-1 w-15 rounded-sm bg-neutral-200 p-1"
                bind:value={pageSizeString}
                onchange={changePageSize}
            >
                <option value="5">5</option>
                <option value="10">10</option>
                <option value="20">20</option>
                <option value="50">50</option>
            </select>
        </label>
    </div>
</div>

{#if classroomListResponse.classrooms === null}
    <p>{m.loading()}</p>
{:else if classroomListResponse.classrooms.length === 0}
    <div class="text-center">
        <p class="flex justify-center text-2xl">{m.noClassroomsFound()}</p>
        <button class="m-2 rounded-sm bg-emerald-500 p-1"
            ><a href={appLink('/classrooms/add')}>{m.classroomAdd()}</a></button
        >
    </div>
{:else}
    <div class="100vw flex flex-col items-center justify-center overflow-auto">
        <table class="border-separate border-spacing-2">
            <thead>
                <tr>
                    <th class="text-left">{m.classroom()}</th>
                    <th class="text-left">{m.description()}</th>
                    <th class="text-left">{m.code()}</th>
                </tr>
            </thead>
            <tbody>
                {#each classroomListResponse.classrooms as classroom (classroom.genData.id)}
                    <tr>
                        <td class="max-w-96 pr-4 wrap-anywhere hover:text-blue-800 hover:underline">
                            <a
                                href={appLink(
                                    `/classrooms/teacher-dashboard?id=${classroom.genData.id}`
                                )}
                                target="_blank">{classroom.data.name}</a
                            >
                        </td>
                        <td class="max-w-125 pr-4 pl-4 align-text-top wrap-anywhere"
                            >{classroom.data.description}</td
                        >
                        <td class="max-w-96 pr-4 pl-4 align-text-top wrap-anywhere"
                            >{classroom.genData.joinCode}</td
                        >
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
            {thisPage}/{calcTotalPages(classroomListResponse?.totalClassrooms ?? 0, pageSizeNumber)}
            <button
                onclick={nextPage}
                disabled={!nextPageExists}
                class="mx-5"
                class:cursor-pointer={nextPageExists}>Next</button
            >
        </div>
    </div>
{/if}
