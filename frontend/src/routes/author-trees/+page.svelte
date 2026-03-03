<script lang="ts">
    //Temporarily hardcoded
    const values = [
        {name: 'You', title: 'Germany in the stone ages had some interesting ideas, but what if this title is realy long, like REALLY REALLY LOOOOOOOOOOOOONG :)', lastEdited: '2026-03-01 13:05:67', published: 'True'},

        {name: 'Theo van den Bogaart', title: 'Je kan de boom in!', lastEdited: '2025-09-14 11:39:59', published: 'True'}
    ]
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

