<script lang="ts">
	async function get_something_from_api() {
		// TODO: probably want some function to make the api calls,
		// so that we don't hardcode '/api/' everywhere
		let api_response = await fetch('/api/json');
		let content = await api_response.json();
		return content;
	}
</script>

{#await get_something_from_api()}
	<p>waiting on api...</p>
{:then json_object}
	<p>Received: {JSON.stringify(json_object)}</p>
{:catch error}
	<p style="color: red">{error.message}</p>
{/await}
