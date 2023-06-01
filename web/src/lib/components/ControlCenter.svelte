<script lang="ts">
    import ControlCenterGroup from "./ControlCenterGroup.svelte";
    import ControlCenterButton from "./ControlCenterButton.svelte";
    import ControlCenterTextbox from "./ControlCenterTextbox.svelte";
    import { createEventDispatcher } from "svelte";
    import { controller } from "$lib/stores/controller";
	import { onMount } from "svelte";

    // Binders to DOM-elements
    let containerDiv: HTMLDivElement;
    let simdev: string;

    // Toggle attachers
    let deviceToggleAttach = 1;

    // Feedback message binders
    let benchmarkCorrectnessFeedback = "Start benchmark to receive feedback";

    // Initialise wasm and then show the controls
    onMount(async () => {
        // Initialisation before the controlcenter is shown to the user
        containerDiv.style.display = "block";
    })

</script>


<div id="container" bind:this={containerDiv}>

    <ControlCenterGroup title="Device selection" columns={2}>
        <!-- <ControlCenterButton text="CPU"
                            type="toggle"
                            bind:toggleAttach={deviceToggleAttach}
                            toggleAttachId={0}
                            columnSpan={2}
                            on:toggleOn={() => {controller.selectSimulationDevice("cpu");}}/> -->
        <ControlCenterButton text="GPU"
                            type="toggle"
                            bind:toggleAttach={deviceToggleAttach}
                            toggleAttachId={1}
                            columnSpan={2}
                            on:toggleOn={() => {controller.selectSimulationDevice("gpu");}}/>
        <ControlCenterButton text="NCHEM"
                            type="toggle"
                            bind:toggleAttach={deviceToggleAttach}
                            toggleAttachId={0}
                            columnSpan={2}
                            on:toggleOn={() => {controller.selectSimulationDevice("nchem");}}/>
        {#if $controller}
            <ControlCenterTextbox bind:text={$controller.cpuIterations} columnSpan={4} />
            <ControlCenterTextbox bind:text={$controller.gpuIterations} columnSpan={4} />
        {/if}
    </ControlCenterGroup>

    <ControlCenterGroup title="Debugging controls">
        <ControlCenterButton text="Clear grid" on:click={() => {controller.clearGrid();}}/>
        <ControlCenterButton text="Spread chemicals randomly" on:click={() => {controller.randomlySpreadChemicals(3);}}/>
        <ControlCenterButton text="Run iteration" on:click={() => {controller.runIteration();}}/>
        <ControlCenterButton text="Run 5 iterations" on:click={() => {controller.run5Iterations();}}/>
        <ControlCenterButton text="Place a patch" on:click={() => {controller.generatePatch();}}/>
    </ControlCenterGroup>

    <ControlCenterGroup title="Benchmarks: CPU vs. GPU output" columns={4}>
        <ControlCenterButton text="Compare now" on:click={async () => {benchmarkCorrectnessFeedback = await controller.compareCPUvsGPUNow();}} columnSpan={1}/>
        <ControlCenterButton text="Compare after catch-up" on:click={async () => {benchmarkCorrectnessFeedback = await controller.compareCPUvsGPUAfterCatchUp();}} columnSpan={1}/>
        <ControlCenterButton text="GPU Shader Benchmark" on:click={async () => {benchmarkCorrectnessFeedback = await controller.benchmarkGPUShaderIncrement();}} columnSpan={1}/>
    </ControlCenterGroup>

    <ControlCenterGroup columns={1}>
        <ControlCenterTextbox bind:text={benchmarkCorrectnessFeedback} columnSpan={4} />
    </ControlCenterGroup>

    <!-- <button on:click={() => {controller.clearGrid();}}>Clear grid</button><br>
    <button on:click={() => {controller.randomlySpreadChemicals(2);}}>Randomly spread 2 chemicals</button><br>
    <button on:click={() => {controller.runIteration();}}>Next iteration</button><br>
    <button on:click={() => {controller.run5Iterations();}}>Run 5 iterations</button><br>
    Select simulation device: <select name="simdev" bind:value={simdev} on:change={() => {controller.selectSimulationDevice(simdev);}}>
        <option value="gpu">GPU</option>
        <option value="cpu">CPU - Multi threaded</option>
    </select> -->

</div>


<style>

    div#container {
        width: 100%;

        display: none;
    }

</style>