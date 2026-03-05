<script lang="ts">
    //import * as user from '$lib/backend/users'
    import { apiCallGetTreesFromUser } from "$lib/backend/trees"

    const values = apiCallGetTreesFromUser();
</script>



<div class="flex h-32 items-center justify-center">
    <table style="width:100%">
        <thead>
        <tr>
            <th style="width:20%">Made By</th>
            <th style="width:50%">Title</th>
            <th style="width:15%">Last Edited</th>
            <th style="width:10%">Published</th>
        </tr>
        </thead>
        <tbody>
            {#each values as value (value.title)} <!-- Do not know why the value.title works, but it does -->
            <tr>
                <td>{value.name}</td>
                <td>{value.title}</td>
                <td>{value.lastEdited}</td>
                <td>{value.published}</td>
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
 - [x] Generate table from data
 - [ ] Make the table a bit more functional -> Language refrences for table headers.
 Then, part 2:
 - [ ] Clicking on the table to get a link to view the tree <- these links don't exist yet, so for now a 404 page would suffice
 -->
