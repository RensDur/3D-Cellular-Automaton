<script lang="ts">
	import type { BatchEntry } from "$lib/classes/BatchEntry";
	import { dashboardController } from "$lib/stores/dashboardController";


    export let batchEntry: BatchEntry;

</script>





Vary <select bind:value={batchEntry.species}>
{#each $dashboardController?.species as s, i}
    <option value={String(i)}>Species {i}</option>
{/each}
</select> - <select bind:value={batchEntry.chemical}>
    <option value="promotor">Promotor</option>
    <option value="demotor">Demotor</option>
</select> - <select bind:value={batchEntry.variable}>
    <option value="range">Range</option>
    <option value="influence">Influence</option>
</select> between <input bind:value={batchEntry.min} type="number"> and <input bind:value={batchEntry.max} type="number"> with steps of <input bind:value={batchEntry.step} type="number" min="0.001" max="10" step="0.001">.



<style>

    input {
        width: 60px;
    }

</style>