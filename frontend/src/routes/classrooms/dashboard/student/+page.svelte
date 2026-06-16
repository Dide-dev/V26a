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
    import { type TreeMetadata } from '$lib/api-wrapper/trees';
    import LessonList from '../LessonList.svelte';
    import { type ClassroomId } from '$lib/api-wrapper/classrooms';

    type ClassroomStudentDashboardData = {
        metaData: ClassroomMetaData;
        lessons: LessonWithTrees[];
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

    const classroomStudentDashboardData: ClassroomStudentDashboardData = {
        metaData: {
            id: 1,
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
            }
        ]
    };

    let id: ClassroomId | null = $state(null);
    //let classroomStudentDashboardData: ClassroomTeacherDashboardData | null = $state(null);

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

        // classroomStudentDashboardData = await getClassroomStudentDashboard(id);
    });
</script>

<div class="flex w-7/8 max-w-xl flex-col">
    {#if classroomStudentDashboardData === null}
        <p class="w-full text-center text-green-800">{m.loadingDashboard()}</p>
    {:else}
        <h1 class="page__title">
            {m.classroomDashboard()}: {classroomStudentDashboardData.metaData.name}
        </h1>

        {#if classroomStudentDashboardData.metaData.description}
            <p class="pb-2 font-semibold wrap-anywhere text-neutral-600">
                {classroomStudentDashboardData.metaData.description}
            </p>
        {/if}

        <LessonList lessons={classroomStudentDashboardData.lessons} isTeacher={false} />
    {/if}
</div>
