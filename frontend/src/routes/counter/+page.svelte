<!--
This program has been developed by students from the bachelor Computer Science at
Utrecht University within the Software Project course.
© Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import * as demo from '$lib/backend/demo';

    let backendPromise = $state(demo.getJsonTest());

    async function increment() {
        await demo.counterIncrement();
        return await demo.getJsonTest();
    }
    async function add(amount: number) {
        await demo.counterAdd({ add: amount });
        return await demo.getJsonTest();
    }
</script>

<div class="flex h-32 items-center justify-center">
    {#await backendPromise}
        <p>waiting on backend...</p>
    {:then backend_state}
        <div class="items-center">
            <p>Current count: {backend_state.b}</p>

            <div class="flex gap-2">
                <button
                    class="rounded-sm bg-emerald-500 p-1"
                    onclick={() => (backendPromise = increment())}
                >
                    Increment!
                </button>

                <button
                    class="rounded-sm bg-emerald-500 p-1"
                    onclick={() => (backendPromise = add(100_000_000))}
                >
                    Increment by 100_000_000!
                </button>
            </div>
        </div>
    {/await}
</div>
