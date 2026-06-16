<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { type TreeMetadata } from '$lib/api-wrapper/trees';
    import { Accordion, AccordionItem } from 'flowbite-svelte';
    import TreeListEntry from '$lib/lists/TreeListEntry.svelte';
    import AddTree from './AddTree.svelte';

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

    let { lessons, isTeacher }: { lessons: LessonWithTrees[]; isTeacher: boolean } = $props();
</script>

<Accordion flush>
    {#each lessons as lesson (lesson.metadata.id)}
        <AccordionItem>
            {#snippet header()}
                <div class="flex w-full items-center justify-between">
                    <div class="flex flex-col">
                        <span class="font-semibold">
                            {lesson.metadata.name}
                        </span>

                        {#if lesson.metadata.description}
                            <span class="text-sm text-neutral-600">
                                {lesson.metadata.description}
                            </span>
                        {/if}
                    </div>

                    {#if isTeacher}
                        <AddTree lessonId={lesson.metadata.id} />
                    {/if}
                </div>
            {/snippet}

            {#each lesson.trees as tree (tree.genData.id)}
                <TreeListEntry {tree} expanded={false} />
            {/each}
        </AccordionItem>
    {/each}
</Accordion>
