<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { controller } from "$lib/stores/controller";
	import { onMount } from "svelte";

    // Binders to DOM-elements
    let containerDiv: HTMLDivElement;
    let dcRange: number;
    let dcInfluence: number;
    let ucRange: number;
    let ucInfluence: number;

    // Initialise wasm and then show the controls
    onMount(async () => {
        // dcRange = $controller.get_dc_range();
        // dcInfluence = $controller.get_dc_influence();
        // ucRange = $controller.get_uc_range();
        // ucInfluence = $controller.get_uc_influence();

        containerDiv.style.display = "block";
    })

</script>


<div id="container" bind:this={containerDiv}>
    <button on:click={() => {controller.clearGrid();}}>Clear grid</button>
    <button on:click={() => {controller.randomlySpreadChemicals(2);}}>Randomly spread 2 chemicals</button>
    <button on:click={() => {controller.runIteration();}}>Next iteration</button>
    <button on:click={() => {controller.run5Iterations();}}>Run 5 iterations</button>
    
    <br>DC Range:       <input bind:value={dcRange}     on:change={() => {controller.updateDCRange(dcRange)}} type="number">
    <br>DC Influence:   <input bind:value={dcInfluence} on:change={() => {controller.updateDCInfluence(dcInfluence)}} type="number">
    <br>UC Range:       <input bind:value={ucRange}     on:change={() => {controller.updateUCRange(ucRange)}} type="number">
    <br>UC Influence:   <input bind:value={ucInfluence} on:change={() => {controller.updateUCInfluence(ucInfluence)}} type="number">


</div>


<style>

    div#container {
        width: 350px;

        display: none;
    }

</style>