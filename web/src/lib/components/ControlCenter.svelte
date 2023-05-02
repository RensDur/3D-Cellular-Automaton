<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { controller } from "$lib/stores/controller";
	import { onMount } from "svelte";

    // Binders to DOM-elements
    let containerDiv: HTMLDivElement;
    let dcRange: number = 2.3;
    let dcInfluence: number = 1;
    let ucRange: number = 6.01;
    let ucInfluence: number = -0.24;

    $: if ($controller) { controller.updateDCRange(dcRange); }
    $: if ($controller) { controller.updateDCInfluence(dcInfluence); }
    $: if ($controller) { controller.updateUCRange(ucRange); }
    $: if ($controller) { controller.updateUCInfluence(ucInfluence); }

    // Initialise wasm and then show the controls
    onMount(async () => {
        await controller.initialise();
        containerDiv.style.display = "block";
    })

</script>


<div id="container" bind:this={containerDiv}>
    <button on:click={() => {controller.clearGrid();}}>Clear grid</button>
    <button on:click={() => {controller.randomlySpreadChemicals(2);}}>Randomly spread 2 chemicals</button>
    <button on:click={() => {controller.runIteration();}}>Next iteration</button>
    
    <br>DC Range: <input bind:value={dcRange} type="number">
    <br>DC Influence: <input bind:value={dcInfluence} type="number">
    <br>UC Range: <input bind:value={ucRange} type="number">
    <br>UC Influence: <input bind:value={ucInfluence} type="number">


</div>


<style>

    div#container {
        width: 350px;

        display: none;
    }

</style>