<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render the data of a classroom for in a list.

    ## Params
    - `classroom: ClassroomMetaData` - The data of the tree to display.
    - `isTeacher: boolean` - Whether the entry should link to a teacher dashboard.
    - `allClassroomUserInfos`: AllClassroomUserInfos | null` - Necessary to display the assigned user number for the student list.
    - `expanded: boolean` - Bindable flag whether the entry displays all its contents.
-->

<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import type { ClassroomId, ClassroomMetaData } from '$lib/api-wrapper/classrooms';
    import { appLink, timeFormatString } from '$lib';
    import AnimatedUnderline from '$lib/AnimatedUnderline.svelte';
    import type { AllClassroomUserInfos } from '$lib/api-wrapper/sessions';

    let timeFormat = new Intl.DateTimeFormat(timeFormatString);

    let {
        classroom,
        isTeacher,
        allClassroomUserInfos = $bindable(),
        expanded = $bindable()
    }: {
        classroom: ClassroomMetaData;
        isTeacher: boolean;
        allClassroomUserInfos: AllClassroomUserInfos | null;
        expanded: boolean;
    } = $props();

    function getNumber(classroomId: ClassroomId): number | string {
        // Will never happen unless user manually deletes his cookie
        if (allClassroomUserInfos === null) return '';

        let classroomUserInfo = allClassroomUserInfos.sessions.find(
            (classroom) => classroom.id === classroomId
        );
        // Will never happen unless user manually changes his cookie
        if (classroomUserInfo === undefined) return '';

        return classroomUserInfo.num;
    }
</script>

<div class="list__entry">
    <div class="flex w-full flex-row justify-between gap-2">
        <div class="flex max-w-2/3 flex-col">
            <span class="flex gap-1 *:*:inline">
                <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                    <a
                        href={appLink(`/classrooms/${classroom.id}${isTeacher ? '/edit' : ''}`)}
                        target="_blank"
                        class={['line-clamp-1 w-fit flex-1 break-all text-ellipsis']}
                        title={m.classroomDashboard()}
                    >
                        <strong>{classroom.name}</strong>
                    </a>
                </AnimatedUnderline>
                <span class="font-semibold text-neutral-500">#{classroom.id}</span>
                <span class="flex-auto grow-100"></span>
            </span>

            <div
                class={[
                    (expanded || classroom.description.length <= 40) && 'inline',
                    !expanded && 'flex overflow-hidden'
                ]}
            >
                {#if classroom.description.length > 0}
                    <span
                        class={[
                            !expanded && 'line-clamp-1',
                            'w-fit flex-1 wrap-break-word text-ellipsis'
                        ]}
                    >
                        {classroom.description}
                    </span>
                {/if}
                {#if classroom.description.length > 40}
                    <button
                        onclick={() => {
                            expanded = !expanded;
                        }}
                        class={[classroom.description.length <= 40 && 'block', 'text__link']}
                    >
                        {expanded ? m.showLess() : m.showMore()}
                    </button>
                {/if}
            </div>
        </div>

        <div class="max-w-1/3 text-right">
            <p class="font-semibold">{timeFormat.format(new Date(classroom.createdAt))}</p>
            {#if !isTeacher}
                <p>{m.usernumber()}: {getNumber(classroom.id)}</p>
            {/if}
        </div>
    </div>
</div>
