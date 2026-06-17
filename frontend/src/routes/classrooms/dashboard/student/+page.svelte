<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { m } from '$lib/paraglide/messages.js';
    import { page } from '$app/state';
    import LessonList from '../LessonList.svelte';
    import {
        type ClassroomId,
        type ClassroomStudentDashboardData,
        getClassroomStudentDashboard
    } from '$lib/api-wrapper/classrooms';

    let id: ClassroomId | null = $state(null);
    let classroomStudentDashboardData: ClassroomStudentDashboardData | null = $state(null);

    onMount(async () => {
        const pageId = page.url.searchParams.get('id');

        if (!pageId) {
            // Change to classroom join page, but that page does not yet exist.
            await goto(appLink('/'));
            return;
        }

        id = Number(pageId);

        if (Number.isNaN(id)) {
            // Change to classroom join page, but that page does not yet exist.
            await goto(appLink('/'));
            return;
        }

        classroomStudentDashboardData = await getClassroomStudentDashboard(id);
    });

    async function reloadDashboard() {
        if (id === null) return;

        classroomStudentDashboardData = await getClassroomStudentDashboard(id);
    }
</script>

<div class="flex w-7/8 max-w-xl flex-col">
    {#if classroomStudentDashboardData === null}
        <p class="w-full text-center text-green-800">{m.loadingDashboard()}</p>
    {:else}
        <h1 class="page__title">
            {m.classroomDashboard()}: {classroomStudentDashboardData.metadata.name}
        </h1>

        {#if classroomStudentDashboardData.metadata.description}
            <p class="pb-2 font-semibold wrap-anywhere text-neutral-600">
                {classroomStudentDashboardData.metadata.description}
            </p>
        {/if}

        <LessonList
            lessons={classroomStudentDashboardData.lessons}
            isTeacher={false}
            onTreeAdded={reloadDashboard}
        />
    {/if}
</div>
