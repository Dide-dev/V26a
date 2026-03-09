<script lang="ts">
    let { acceptCard, declineCard }: {
        acceptCard: (text: string) => void;
        declineCard: () => void;
    } = $props();

    let text = $state('');
</script>

<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <div class="bg-white rounded p-4 flex flex-col gap-2">
        <textarea class="border rounded p-1" placeholder="Enter card text..." bind:value={text}></textarea>
        <div class="flex gap-2 justify-end">
            <button onclick={declineCard}>Cancel</button>
            <button onclick={() => acceptCard(text)} disabled={!text}>Add</button>
        </div>
    </div>
</div>