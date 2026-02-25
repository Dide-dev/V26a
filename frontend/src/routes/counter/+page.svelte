<script lang="ts">
	import * as demo from '$lib/backend/demo';

	let backend_promise = $state(demo.get_json_test());

	async function increment() {
		await demo.counter_increment();
		return await demo.get_json_test();
	}
	async function add(amount: number) {
		await demo.counter_add({ add: amount });
		return await demo.get_json_test();
	}
</script>



<div class="flex h-32 items-center justify-center">
	{#await backend_promise}
		<p>waiting on backend...</p>
	{:then backend_state}
		<div class="items-center">
			<p>Current count: {backend_state.b}</p>

			<div class="flex gap-2">
				<button
					class="rounded-sm bg-emerald-500 p-1"
					onclick={() => (backend_promise = increment())}
				>
					Increment!
				</button>

				<button
					class="rounded-sm bg-emerald-500 p-1"
					onclick={() => (backend_promise = add(100_000_000))}
				>
					Increment by 100_000_000!
				</button>
			</div>
		</div>
	{/await}
</div>
