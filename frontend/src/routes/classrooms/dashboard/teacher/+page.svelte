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
    import { getUserInfo, type UserInfo } from '$lib/api-wrapper/users';
    import { m } from '$lib/paraglide/messages.js';
    import { page } from '$app/state';
    import { type TreeMetadata } from '$lib/api-wrapper/trees';
    import LessonList from '../LessonList.svelte';
    import AddLesson from './AddLesson.svelte';
    import type { ClassroomId } from '$lib/api-wrapper/classrooms';

    type ClassroomTeacherDashboardData = {
        metaData: ClassroomMetaData;
        lessons: LessonWithTrees[];
        joinCode: string;
    };

    type ClassroomMetaData = {
        id: number;
        name: string;
        description: string;
        createdAt: Date;
        ownerId: number;
    };

    type LessonWithTrees = {
        metadata: LessonMetaData;
        trees: TreeMetadata[];
    };

    type LessonMetaData = {
        id: number;
        order_number: number;
        createdAt: Date;
        name: string;
        description: string;
        published: boolean;
    };

    const classroomTeacherDashboardData: ClassroomTeacherDashboardData = {
        metaData: {
            id: 2,
            name: 'Frontend Development Bootcamp',
            description: 'Een klaslokaal voor het leren van moderne frontend-technologieën.',
            createdAt: new Date('2025-01-15T09:00:00Z'),
            ownerId: 1001
        },
        lessons: [
            {
                metadata: {
                    id: 101,
                    order_number: 1,
                    createdAt: new Date('2025-01-20T10:00:00Z'),
                    name: 'Introductie tot HTML',
                    description: 'Leer de basis van HTML en documentstructuren.',
                    published: true
                },
                trees: [
                    {
                        data: {
                            title: 'HTML Basisstructuur',
                            languageCode: 'nl',
                            tags: ['html', 'introductie'],
                            description:
                                'Een overzicht van de basisstructuur van een HTML-document.',
                            subjectId: 1,
                            bookName: 'Web Development Basics',
                            bookEdition: '3e editie',
                            bookAuthor: 'Jan Jansen',
                            bookIsbn: '9781234567890',
                            bookChapter: '1',
                            bookExercise: '1.1',
                            assignmentText: 'Maak een eenvoudige HTML-pagina.',
                            published: true
                        },
                        genData: {
                            id: 5001,
                            ownerId: 1001,
                            createdAt: new Date('2025-01-20T10:30:00Z'),
                            lastModified: new Date('2025-01-22T15:00:00Z'),
                            timesVisited: 42,
                            lastVisited: new Date('2025-02-01T09:15:00Z')
                        },
                        linkedData: {
                            ownerName: 'Jan Jansen',
                            subjectName: 'Web Development',
                            languageName: 'Nederlands',
                            coAuthors: ['Piet Pieters']
                        }
                    }
                ]
            },
            {
                metadata: {
                    id: 102,
                    order_number: 1,
                    createdAt: new Date('2025-01-20T10:00:00Z'),
                    name: 'Introductie tot HTML',
                    description: 'Leer de basis van HTML en documentstructuren.',
                    published: true
                },
                trees: [
                    {
                        data: {
                            title: 'HTML Basisstructuur',
                            languageCode: 'nl',
                            tags: ['html', 'introductie'],
                            description:
                                'Een overzicht van de basisstructuur van een HTML-document.',
                            subjectId: 1,
                            bookName: 'Web Development Basics',
                            bookEdition: '3e editie',
                            bookAuthor: 'Jan Jansen',
                            bookIsbn: '9781234567890',
                            bookChapter: '1',
                            bookExercise: '1.1',
                            assignmentText: 'Maak een eenvoudige HTML-pagina.',
                            published: true
                        },
                        genData: {
                            id: 5001,
                            ownerId: 1001,
                            createdAt: new Date('2025-01-20T10:30:00Z'),
                            lastModified: new Date('2025-01-22T15:00:00Z'),
                            timesVisited: 42,
                            lastVisited: new Date('2025-02-01T09:15:00Z')
                        },
                        linkedData: {
                            ownerName: 'Jan Jansen',
                            subjectName: 'Web Development',
                            languageName: 'Nederlands',
                            coAuthors: ['Piet Pieters']
                        }
                    },
                    {
                        data: {
                            title: 'HTML Basisstructuur',
                            languageCode: 'nl',
                            tags: ['html', 'introductie'],
                            description:
                                'Een overzicht van de basisstructuur van een HTML-document.',
                            subjectId: 1,
                            bookName: 'Web Development Basics',
                            bookEdition: '3e editie',
                            bookAuthor: 'Jan Jansen',
                            bookIsbn: '9781234567890',
                            bookChapter: '1',
                            bookExercise: '1.1',
                            assignmentText: 'Maak een eenvoudige HTML-pagina.',
                            published: true
                        },
                        genData: {
                            id: 5001,
                            ownerId: 1001,
                            createdAt: new Date('2025-01-20T10:30:00Z'),
                            lastModified: new Date('2025-01-22T15:00:00Z'),
                            timesVisited: 42,
                            lastVisited: new Date('2025-02-01T09:15:00Z')
                        },
                        linkedData: {
                            ownerName: 'Jan Jansen',
                            subjectName: 'Web Development',
                            languageName: 'Nederlands',
                            coAuthors: ['Piet Pieters']
                        }
                    }
                ]
            }
        ],
        joinCode: 'ABCDE1'
    };

    let userInfo: UserInfo | null = $state(null);
    let id: ClassroomId | null = $state(null);

    //let classroomTeacherDashboardData: ClassroomTeacherDashboardData | null = $state(null);

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

        // classroomTeacherDashboardData = await getClassroomTeacherDashboard(id);
    });

    async function copyCode() {
        await navigator.clipboard.writeText(classroomTeacherDashboardData.joinCode);
    }
</script>

<div class="flex w-7/8 flex-col">
    {#if classroomTeacherDashboardData === null}
        <p class="w-full text-center text-green-800">{m.loadingDashboard()}</p>
    {:else}
        <h1 class="page__title">
            {m.classroomDashboard()}: {classroomTeacherDashboardData.metaData.name}
        </h1>

        {#if classroomTeacherDashboardData.metaData.description}
            <p class="pb-2 font-semibold wrap-anywhere text-neutral-600">
                {classroomTeacherDashboardData.metaData.description}
            </p>
        {/if}

        <div class="grid grid-cols-[1fr_280px] gap-4">
            <div>
                <LessonList lessons={classroomTeacherDashboardData.lessons} isTeacher={true} />
            </div>

            <aside class="space-y-4">
                <div class="rounded border p-3">
                    <p class="text-sm text-neutral-500">Join code</p>

                    <div class="flex items-center justify-between">
                        <code class="font-mono">{classroomTeacherDashboardData.joinCode}</code>

                        <button
                            class="cursor-pointer rounded-md border-2 border-current bg-neutral-200 p-1 text-lg font-bold text-neutral-600 transition-all duration-200"
                            onclick={copyCode}>{m.copyCode()}</button
                        >
                    </div>
                </div>
                <AddLesson classroomId={classroomTeacherDashboardData.metaData.id} />
            </aside>
        </div>
    {/if}
</div>
