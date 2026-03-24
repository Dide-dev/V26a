<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
</script>

<div class="flex h-32 items-center justify-center">
    <p>{m.helloWorld({ name: 'wortel' })}</p>
</div>
