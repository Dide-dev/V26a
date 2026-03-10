<script lang="ts">
    import { m } from '$lib/paraglide/messages'
    import { apiCallGetTreesFromUser } from "$lib/backend/trees"

    const header: string[] = JSON.parse(m.table_header());
    const values = apiCallGetTreesFromUser();

    function parseBooleanToString(value: boolean) {
        if (value) return header[4]; // Yes
        return header[5]; // No
    }
</script>



<div class="mx-4 h-24">
    <table class="w-full">
        <thead>
        <tr>
            <th class="text-left">{header[0]}</th>
            <th class="text-left">{header[1]}</th>
            <th class="text-left">{header[2]}</th>
            <th class="text-left">{header[3]}</th>
        </tr>
        </thead>
        <tbody>
            {#each values as value (value.title)} <!-- Do not know why the value.title works, but it does -->
            <tr>
                <td class="pr-4">{value.name}</td>
                <td class="pr-4">{value.title}</td>
                <td class="pr-4">{(value.lastEdited).toLocaleString()}</td> <!-- This formats the date in the local timezone in the form: dd-mm-yyyy, hh:mm:ss -->
                <td >{parseBooleanToString(value.published)}</td>
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
 - [ ] Make the table a bit more functional.
    - [x] Have it not weirdly go all over the place when resizing the window, Don't know why it is fixed.
    - [x] Have the headers be not centred,
    - [x] Have the date format be more legible.
 Then, part 2:
 - [ ] Clicking on the table to get a link to view the tree <- these links don't exist yet, so for now a 404 page would suffice
 -->
