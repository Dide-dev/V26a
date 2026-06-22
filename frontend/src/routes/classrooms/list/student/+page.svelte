<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import {
        type ClassroomListResponse,
        getAllClassroomsStudent
    } from '$lib/api-wrapper/classrooms';
    import { appLink } from '$lib';
    import { onMount } from 'svelte';
    import { calcTotalPages, type PaginationOptions } from '$lib/pagination';
    import {
        getAllClassroomUserInfos,
        type AllClassroomUserInfos
    } from '$lib/api-wrapper/sessions';
    import ListContainer from '$lib/lists/ListContainer.svelte';
    import ClassroomListEntry from '$lib/lists/ClassroomListEntry.svelte';
    import { PlusOutline } from 'flowbite-svelte-icons';

    // This cannot be done with the {#await ...} in svelte,
    // because then updating it browser-side for classroom deletion is not possible.
    let classroomListResponse: ClassroomListResponse = $state({
        totalClassrooms: 0,
        classrooms: []
    });

    let allClassroomUserInfos: AllClassroomUserInfos | null = $state(null);

    let pageSize = $state(10);
    let currentPage = $state(1);
    let totalPages = $derived(
        calcTotalPages(classroomListResponse?.totalClassrooms ?? 0, pageSize)
    );

    onMount(async () => {
        await loadClassrooms();
    });

    async function loadClassrooms() {
        allClassroomUserInfos = getAllClassroomUserInfos();

        let paginationOptions: PaginationOptions = {
            page: currentPage,
            size: pageSize
        };
        if (allClassroomUserInfos !== null) {
            classroomListResponse = await getAllClassroomsStudent(paginationOptions);
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

<ListContainer
    bind:currentPage
    {totalPages}
    {handlePageChange}
    bind:pageSize
    {handlePageSizeChange}
>
    {#snippet title()}
        <h1 class="page__title flex gap-2 *:*:inline">
            {m.joinedClassrooms()}
            <a
                title={m.addTree()}
                aria-label={m.addTree()}
                class="mr-2.5 cursor-pointer transition-all duration-200 ease-in-out hover:bg-neutral-200"
                href={appLink(`/classrooms/join`)}
                target="_blank"
            >
                <PlusOutline class="h-7 w-7 shrink-0" />
            </a>
        </h1>
    {/snippet}
    {#if classroomListResponse?.classrooms === null}
        <p>{m.loading()}</p>
    {:else if classroomListResponse?.classrooms.length === 0}
        <p class="mt-4 font-semibold italic">{m.noClassroomsFound()}</p>
    {:else}
        <div class="list__container">
            {#each classroomListResponse?.classrooms as classroom, i (classroom.id)}
                <ClassroomListEntry
                    {classroom}
                    isTeacher={false}
                    bind:allClassroomUserInfos
                    bind:expanded={entriesExpanded[i]}
                />
            {/each}
        </div>
    {/if}
</ListContainer>
