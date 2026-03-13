<script lang="ts">
    import { page } from '$app/state';

    let treeId = Number(page.params.id);
    console.log('Tree ID:', treeId);
</script>
