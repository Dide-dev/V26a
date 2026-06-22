<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { Accordion, AccordionItem } from 'flowbite-svelte';
    import TreeListEntry from '$lib/lists/TreeListEntry.svelte';
    import AddTree from './AddTree.svelte';
    import { type LessonWithTrees } from '$lib/api-wrapper/classrooms';

    let {
        lessons,
        isTeacher,
        onTreeAdded
    }: { lessons: LessonWithTrees[]; isTeacher: boolean; onTreeAdded: () => Promise<void> } =
        $props();

    function handleDelete() {}
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
                            <span class="text-sm wrap-anywhere text-neutral-600">
                                {lesson.metadata.description}
                            </span>
                        {/if}
                    </div>

                    {#if isTeacher}
                        <AddTree lessonId={lesson.metadata.id} {onTreeAdded} />
                    {/if}
                </div>
            {/snippet}

            {#each lesson.trees as tree (tree.genData.id)}
                <TreeListEntry {tree} expanded={false} {handleDelete} usedForClassroom={true} />
            {/each}
        </AccordionItem>
    {/each}
</Accordion>
