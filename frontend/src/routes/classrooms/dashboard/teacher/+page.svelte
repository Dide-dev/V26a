<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { redirectLogin } from '$lib/api-wrapper';
    import { getUserInfo, type UserInfo } from '$lib/api-wrapper/sessions';
    import { m } from '$lib/paraglide/messages.js';
    import { page } from '$app/state';
    import LessonList from '../LessonList.svelte';
    import AddLesson from './AddLesson.svelte';
    import type { ClassroomId, ClassroomTeacherDashboardData } from '$lib/api-wrapper/classrooms';
    import { getClassroomTeacherDashboard } from '$lib/api-wrapper/classrooms';

    let userInfo: UserInfo | null = $state(null);
    let id: ClassroomId | null = $state(null);

    let classroomTeacherDashboardData: ClassroomTeacherDashboardData | null = $state(null);

    onMount(async () => {
        userInfo = getUserInfo();
        if (userInfo === null) {
            await redirectLogin(page.url);
            return;
        }

        const pageId = page.url.searchParams.get('id');

        if (!pageId) {
            await goto(appLink('/classrooms/add'));
            return;
        }

        id = Number(pageId);

        if (Number.isNaN(id)) {
            await goto(appLink('/classrooms/add'));
            return;
        }

        classroomTeacherDashboardData = await getClassroomTeacherDashboard(id);
    });

    async function copyCode() {
        await navigator.clipboard.writeText(classroomTeacherDashboardData!.joinCode);
    }

    async function reloadDashboard() {
        if (id === null) return;

        classroomTeacherDashboardData = await getClassroomTeacherDashboard(id);
    }
</script>

<div class="flex w-7/8 flex-col">
    {#if classroomTeacherDashboardData === null}
        <p class="w-full text-center text-green-800">{m.loadingDashboard()}</p>
    {:else}
        <h1 class="page__title">
            {m.classroomDashboard()}: {classroomTeacherDashboardData.metadata.name}
        </h1>

        {#if classroomTeacherDashboardData.metadata.description}
            <p class="pb-2 font-semibold wrap-anywhere text-neutral-600">
                {classroomTeacherDashboardData.metadata.description}
            </p>
        {/if}

        <div class="grid grid-cols-[1fr_280px] gap-4">
            <div>
                <LessonList
                    lessons={classroomTeacherDashboardData.lessons}
                    isTeacher={true}
                    onTreeAdded={reloadDashboard}
                />
            </div>

            <aside class="space-y-4">
                <div class="rounded border p-3">
                    <p class="text-sm text-neutral-500">Join code</p>

                    <div class="flex items-center justify-between">
                        <code class="font-mono">{classroomTeacherDashboardData.joinCode}</code>

                        <button
                            class="cursor-pointer rounded-md border-2 border-neutral-600 bg-neutral-200 p-1 text-lg font-bold text-black transition-all duration-200 ease-in-out hover:bg-neutral-600 hover:text-white"
                            onclick={copyCode}>{m.copyCode()}</button
                        >
                    </div>
                </div>
                <AddLesson
                    classroomId={classroomTeacherDashboardData.metadata.id}
                    onLessonAdded={reloadDashboard}
                />
            </aside>
        </div>
    {/if}
</div>
