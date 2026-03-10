<script lang="ts">
    import { m } from '$lib/paraglide/messages'
    import { apiCallGetTreesFromUser } from "$lib/backend/trees"
    import img from '$lib/assets/compose.svg'

    const header: string[] = JSON.parse(m.table_header());
    const values = apiCallGetTreesFromUser();

    function parseBooleanToString(value: boolean) {
        if (value) return header[4]; // Yes
        return header[5]; // No
    }
</script>



<div class="min-w-16 relative md:left-10 lg:left-20 mx-4 h-24">
    <table >
        <thead>
        <tr>
            <th class="text-left">{header[0]}</th>
            <th class="text-left">{header[1]}</th>
            <th class="text-left">{header[2]}</th>
            <th class="text-left">{header[3]}</th>
            <th class="text-left"></th>
        </tr>
        </thead>
        <tbody>
            {#each values as value (value.treeID)}
            <tr>
                <td class="pr-4">{value.name}</td>
                <td class="pr-4 hover:text-blue-800 hover:underline"><a href="/tree-viewer/{value.treeID}">{value.title}</a></td>
                <td class="pr-4">{(value.lastEdited).toLocaleString()}</td> <!-- This formats the date in the local timezone in the form: dd-mm-yyyy, hh:mm:ss -->
                <td >{parseBooleanToString(value.published)}</td>
                <td class="hover:fill-amber-700"><a href="/edit"><img src="{img}" alt="edit" width="20" height="20" class="fill-amber-400"></a></td>
            </tr>
            {/each}
        </tbody>
    </table>
</div>

<!-- TODO:
 - [x] Make a API-request for trees (Frontend) <- how can you obtain the appropriate user_id? -> Backend
 - [ ] With user_id request all trees from the trees table which the user can edit (Backend)
 - [ ] Make API call for all trees for which the owner is not the same user as user_id (Frontend or Backend?)
 - [ ] Lookup for each tree what the name of the owner is, if it is not the same as the user_id (Backend)
 - [x] Compile list of all trees with data (Frontend) <- mock API already does this now.
 - [x] Generate table from data + Language reference headers.
 - [x] Make the table a bit more functional.
 Then, part 2:
 - [x] Clicking on the table to get a link to view the tree
    - [ ] Check if links actually work. This requires different story to be finished.
 -->
