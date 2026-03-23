<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->
<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import { getTreesFromUser } from '$lib/backend/trees';
    import icon from '$lib/assets/editIcon.svg?raw';
    import { appLink } from '$lib/index';

    const values = getTreesFromUser();

    function parseBooleanToString(value: boolean) {
        if (value) return m.yes();
        return m.no();
    }
    let thisPage = $state(1);
    let totalPages = $state(10);
</script>

{#if values.length > 0}
    <!-- For dynamically formatting the entire css has to be redone. -->
    <div class="relative left-1/4 mx-4 h-24 max-w-200">
        <table class="border-separate border-spacing-2">
            <thead>
                <tr>
                    <th class="text-left">{m.madeBy()}</th>
                    <th class="text-left">{m.title()}</th>
                    <th class="text-left">{m.lastEdited()}</th>
                    <th class="text-left">{m.published()}</th>
                    <!-- Edit button -->
                    <th class="text-left"></th>
                </tr>
            </thead>
            <tbody>
                {#each values as value (value.treeID)}
                    <tr>
                        <td class="pr-4">{value.username}</td>
                        <td class="max-w-96 pr-4 hover:text-blue-800 hover:underline">
                            <a href={appLink(`/tree-viewer?id=${value.treeID}/`)} target="_blank"
                                >{value.title}</a
                            >
                        </td>
                        <td class="pr-4">{value.lastEdited.toLocaleString()}</td>
                        <!-- This formats the date in the local timezone in the form: dd-mm-yyyy, hh:mm:ss -->
                        <td class="pr-4">{parseBooleanToString(value.published)}</td>
                        <td class="min-w-2 hover:fill-blue-800">
                            <a href={appLink('/')} target="_blank" aria-label="edit">
                                <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                                {@html icon}
                            </a>
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{:else}
    <p class="flex justify-center text-2xl">{m.noTreesFound()}</p>
{/if}

<div class="bottom-2 flex justify-center">
    <button>Prev</button>
    {thisPage}/{totalPages}
    <button>Next</button>
</div>
