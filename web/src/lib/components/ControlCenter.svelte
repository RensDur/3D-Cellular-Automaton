<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { controller } from "$lib/stores/controller";
	import { onMount } from "svelte";

    // Binders to DOM-elements
    let containerDiv: HTMLDivElement;
    let simdev: string;

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
    <button on:click={() => {controller.clearGrid();}}>Clear grid</button><br>
    <button on:click={() => {controller.randomlySpreadChemicals(2);}}>Randomly spread 2 chemicals</button><br>
    <button on:click={() => {controller.runIteration();}}>Next iteration</button><br>
    <button on:click={() => {controller.run5Iterations();}}>Run 5 iterations</button><br>
    Select simulation device: <select name="simdev" bind:value={simdev} on:change={() => {controller.selectSimulationDevice(simdev);}}>
        <option value="gpu">GPU</option>
        <option value="cpu">CPU - Multi threaded</option>
    </select>

</div>


<style>

    div#container {
        width: 350px;

        display: none;
    }

</style>