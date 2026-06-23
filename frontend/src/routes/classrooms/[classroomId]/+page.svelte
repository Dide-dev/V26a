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
    import LessonList from './LessonList.svelte';
    import {
        type ClassroomId,
        type ClassroomStudentDashboardData,
        getClassroomStudentDashboard
    } from '$lib/api-wrapper/classrooms';
    import { getClassroomUserInfo, type ClassroomUserInfo } from '$lib/api-wrapper/sessions';

    let { params } = $props();

    let classroomUserInfo: ClassroomUserInfo | null = $state(null);
    let id: ClassroomId = $derived(Number(params.classroomId));
    let classroomStudentDashboardData: ClassroomStudentDashboardData | null = $state(null);

    onMount(async () => {
        if (Number.isNaN(id)) {
            await goto(appLink('/classrooms/join'));
            return;
        }

        classroomUserInfo = getClassroomUserInfo(id);

        if (classroomUserInfo === null) {
            await goto(appLink('/classrooms/join'));
            return;
        }

        classroomStudentDashboardData = await getClassroomStudentDashboard(id);
    });

    async function reloadDashboard() {
        if (id === null) return;

        classroomStudentDashboardData = await getClassroomStudentDashboard(id);
    }
</script>

<div class="flex w-5/8 flex-col">
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

        <div class="grid grid-cols-[1fr_160px] gap-4">
            <div>
                <LessonList
                    lessons={classroomStudentDashboardData.lessons}
                    isTeacher={false}
                    onTreeAdded={reloadDashboard}
                />
            </div>

            <aside class="space-y-4">
                <div class="rounded border p-3">
                    <p class="text-sm text-neutral-500">{m.usernumber()}</p>
                    <div class="flex items-center justify-between">
                        <code class="font-mono">{classroomUserInfo!.num}</code>
                    </div>
                    <p class="text-sm text-neutral-500">{m.name()}</p>
                    <div class="flex items-center justify-between">
                        <code class="font-mono">{classroomUserInfo!.name}</code>
                    </div>
                </div>
            </aside>
        </div>
    {/if}
</div>
