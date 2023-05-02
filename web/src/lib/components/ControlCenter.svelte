<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { controller } from "$lib/stores/controller";
	import { onMount } from "svelte";

    // Binders to DOM-elements
    let containerDiv: HTMLDivElement;

    // Initialise wasm and then show the controls
    onMount(async () => {
        await controller.initialise();
        containerDiv.style.display = "block";
    })

</script>


<div id="container" bind:this={containerDiv}>
    <button on:click={() => {controller.createEmptyGrid(20);}}>Create empty grid</button>
    <button on:click={() => {controller.randomlySpreadChemicals(2);}}>Randomly spread 2 chemicals</button>
    <button on:click={() => {controller.runIteration();}}>Next iteration</button>
    <button on:click={() => {
        for (let x = 0; x < $controller.size(); ++x) {
            let output = "";
            for (let y = 0; y < $controller.size(); ++y) {
                output += $controller.get(x, y);
            }
            console.log(output);
        }
    }}>Print current grid</button>
</div>


<style>

    div#container {
        width: 350px;

        display: none;
    }

</style>