<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import {
        type ClassroomListResponse,
        getAllClassroomsTeacher
    } from '$lib/api-wrapper/classrooms';
    import { onMount } from 'svelte';
    import { calcTotalPages, type PaginationOptions } from '$lib/pagination';
    import { getUserInfo } from '$lib/api-wrapper/sessions';
    import { redirectLogin } from '$lib/api-wrapper';
    import { page } from '$app/state';
    import ListPagination from '$lib/lists/ListPagination.svelte';
    import ClassroomListEntry from '$lib/lists/ClassroomListEntry.svelte';
    import { appLink } from '$lib';

    // This cannot be done with the {#await ...} in svelte,
    // because then updating it browser-side for classroom deletion is not possible.
    let classroomListResponse: ClassroomListResponse = $state({
        totalClassrooms: 0,
        classrooms: []
    });

    let pageSize = $state(10);
    let currentPage = $state(1);
    let totalPages = $derived(
        calcTotalPages(classroomListResponse?.totalClassrooms ?? 0, pageSize)
    );

    onMount(async () => {
        await loadClassrooms();
    });

    async function loadClassrooms() {
        let userInfo = getUserInfo();

        let paginationOptions: PaginationOptions = {
            page: currentPage,
            size: pageSize
        };
        if (userInfo !== null) {
            classroomListResponse = await getAllClassroomsTeacher(paginationOptions);
        } else {
            await redirectLogin(page.url);
        }
    }

    function handlePageSizeChange() {
        // Update the length of the description shown array, and reset values to false.
        if (pageSize > entriesExpanded.length) {
            entriesExpanded = Array.from({ length: pageSize }, () => false);
        }
        // Move to page 1, to prevent out-of-range page errors.
        currentPage = 1;
        loadClassrooms();
    }

    function handlePageChange() {
        // Reset the expanded flags to false.
        entriesExpanded.fill(false);
        loadClassrooms();
    }

    let entriesExpanded = $state(Array.from({ length: 10 }, () => false));
</script>

<div class="flex w-7/8 max-w-xl flex-col">
    <h1 class="page__title">{m.myClassrooms()}</h1>
    {#if classroomListResponse?.classrooms === null}
        <p>{m.loading()}</p>
    {:else if classroomListResponse?.classrooms.length === 0}
        <p class="mt-4 font-semibold italic">{m.noClassroomsFound()}</p>
        <div class="text-center">
            <button class="m-2 rounded-sm bg-emerald-500 p-1"
                ><a href={appLink('/classrooms/add')}>{m.createClassroom()}</a></button
            >
        </div>
    {:else}
        <div class="flex w-full flex-col items-center justify-center overflow-auto">
            <ListPagination
                bind:currentPage
                {totalPages}
                {handlePageChange}
                bind:pageSize
                {handlePageSizeChange}
            >
                <div class="list__container">
                    {#each classroomListResponse?.classrooms as classroom, i (classroom.id)}
                        <ClassroomListEntry
                            {classroom}
                            isTeacher={true}
                            allClassroomUserInfos={null}
                            bind:expanded={entriesExpanded[i]}
                        />
                    {/each}
                </div>
            </ListPagination>
        </div>
    {/if}
</div>
