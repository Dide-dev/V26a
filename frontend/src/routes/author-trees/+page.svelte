<script lang="ts">
    import { m } from '$lib/paraglide/messages';
    import { apiCallGetTreesFromUser } from '$lib/backend/trees';
    import icon from '$lib/assets/editIcon.svg?raw';
    // Obtain all language sensitive phrases
    const header: string[] = JSON.parse(m.table_header());
    const values = apiCallGetTreesFromUser();

    function parseBooleanToString(value: boolean) {
        if (value) return header[4]; // Yes
        return header[5]; // No
    }

    {
        /* eslint-disable svelte/no-at-html-tags */
    }
</script>

<!-- For dynamically formatting the entire css has to be redone. -->
<div class="relative left-1/4 mx-4 h-24 max-w-200">
    <table class="border-separate border-spacing-2">
        <thead>
            <tr>
                <th class="text-left">{header[0]}</th>
                <!-- Made by -->
                <th class="text-left">{header[1]}</th>
                <!-- Title -->
                <th class="text-left">{header[2]}</th>
                <!-- Last edited -->
                <th class="text-left">{header[3]}</th>
                <!-- Published -->
                <th class="text-left"></th>
                <!-- Edit button -->
            </tr>
        </thead>
        <tbody>
            {#each values as value (value.treeID)}
                <tr>
                    <td class="pr-4">{value.username}</td>
                    <td class="max-w-96 pr-4 hover:text-blue-800 hover:underline"
                        ><a href="/tree-viewer/">{value.title}</a></td
                    >
                    <td class="pr-4">{value.lastEdited.toLocaleString()}</td>
                    <!-- This formats the date in the local timezone in the form: dd-mm-yyyy, hh:mm:ss -->
                    <td class="pr-4">{parseBooleanToString(value.published)}</td>
                    <td class="min-w-2 hover:fill-blue-800">
                        <a href="/" aria-label="edit">
                            {@html icon}
                        </a>
                    </td>
                </tr>
            {/each}
        </tbody>
    </table>
</div>
