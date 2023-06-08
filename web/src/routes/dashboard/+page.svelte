<script lang="ts">
	import { BatchEntry } from "$lib/classes/BatchEntry";
	import { Chemical } from "$lib/classes/Chemical";
    import { Species } from "$lib/classes/Species";


    import MainStageMarchingCubes from "$lib/components/MainStageMarchingCubes.svelte";
    import OrderParameterGraph from "$lib/components/OrderParameterGraph.svelte";
	import BatchEntryRow from "$lib/components/dashboard/BatchEntryRow.svelte";
    import { dashboardController } from "$lib/stores/dashboardController";
	import { onMount } from "svelte";

    let mainStageContainerWidth = 0;
    let mainStageContaienrHeight = 0;

    let orderParameterContainerWidth = 0;
    let orderParameterContainerHeight = 0;



    // DOM Bindings
    let speciesSelector: HTMLSelectElement;
    let removeSpeciesButton: HTMLButtonElement;

    let promotorRangeInput: HTMLInputElement;
    let promotorInfluenceInput: HTMLInputElement;
    let demotorRangeInput: HTMLInputElement;
    let demotorInfluenceInput: HTMLInputElement;

    let batchProgrammingTable: HTMLTableElement;



    // State keeping
    let selectedSpecies: Species | undefined = undefined;
    let numberOfIterations: number = 10;

    let batchEntries: BatchEntry[] = [];

    // METHODS RELATED TO BATCH PROGRAMMING
    



    // METHODS RELATED TO SPECIES
    function updateSelector() {

        let oldValue = speciesSelector.value;

        // Clear all species in the DOM
        speciesSelector.innerHTML = "";

        // Insert all species that are currently in the store
        for (let i = 0; i < $dashboardController?.species.length; i++) {
            let opt = document.createElement("option");
            opt.setAttribute("value", String(i));
            opt.innerHTML = "Species " + String(i);

            speciesSelector.appendChild(opt);
        }

        let opt = document.createElement("option");
        opt.setAttribute("value", String($dashboardController?.species.length));
        opt.innerHTML = "Undif";

        speciesSelector.appendChild(opt);

        speciesSelector.value = oldValue;

        // Show the 'remove species' button at appropriate time
        if ($dashboardController?.species.length > 0 || speciesSelector.value == "") {
            removeSpeciesButton.removeAttribute("disabled");
        } else {
            removeSpeciesButton.setAttribute("disabled", "true");
        }

        // Update the selected species tracker
        if (speciesSelector.value == "") {
            selectedSpecies = undefined;
        } else {
            selectedSpecies = $dashboardController?.species[parseInt(speciesSelector.value)];
        }


        // Based on the currently selected species, update the marching cubes mesh
        dashboardController?.requestMeshForSpecies(parseInt(speciesSelector.value));

        // Update the possibile species that can be selected in the batch programming section
        let oldBe = batchEntries;

        batchEntries = [];

        batchEntries = oldBe;
        
    }

    function insertNewSpecies() {
        $dashboardController?.insertSpecies(new Species(new Chemical(0, 0), new Chemical(0, 0)));

        updateSelector();
    }

    function removeSpecies() {
        $dashboardController?.removeSpecies(parseInt(speciesSelector.value));

        updateSelector();
    }



    onMount(async () => {
        await dashboardController.initialise();

        // Immediately update the selector with the server's configuration of species and chemicals
        updateSelector();
    })

</script>



<div id="wrapper">
    <div class="container" id="stage-container" bind:clientWidth={mainStageContainerWidth} bind:clientHeight={mainStageContaienrHeight}>
        <MainStageMarchingCubes bind:sceneWidth={mainStageContainerWidth} bind:sceneHeight={mainStageContaienrHeight} />
    </div>
    <div class="container" id="order-parameter-container" bind:clientWidth={orderParameterContainerWidth} bind:clientHeight={orderParameterContainerHeight}>
        <OrderParameterGraph bind:windowWidth={orderParameterContainerWidth} bind:windowHeight={orderParameterContainerHeight}/>
    </div>
    <div class="container" id="controls-container">

        <div id="species">
            <p>Select a species</p>
            <select on:change={updateSelector} bind:this={speciesSelector} name="species-select" id="species-select" size="8">

            </select>
            <button on:click={insertNewSpecies}>Insert new species</button>
            <button on:click={removeSpecies} bind:this={removeSpeciesButton} disabled>Remove species</button>
        </div>

        <div id="single-run-dashboard">

            {#if (selectedSpecies == undefined)}
                <p id="no-species-selected">No species selected</p>
            {:else}
                <p id="selected-species">Selected species: {speciesSelector.value}</p>

                <table>
                    <tr>
                        <td>Promotor range</td>
                        <td><input bind:this={promotorRangeInput} type="number" bind:value={selectedSpecies.chemicalA.range}></td>
                    </tr>
                    <tr>
                        <td>Promotor influence</td>
                        <td><input bind:this={promotorInfluenceInput} type="number" bind:value={selectedSpecies.chemicalA.influence}></td>
                    </tr>
                    <tr>
                        <td>Demotor range</td>
                        <td><input bind:this={demotorRangeInput} type="number" bind:value={selectedSpecies.chemicalB.range}></td>
                    </tr>
                    <tr>
                        <td>Demotor influence</td>
                        <td><input bind:this={demotorInfluenceInput} type="number" bind:value={selectedSpecies.chemicalB.influence}></td>
                    </tr>
                </table>

                <span class="space"></span>

                <table>
                    <tr>
                        <td>Number of iterations</td>
                        <td><input type="number" style="width: 100px;" bind:value={numberOfIterations}></td>
                        <td><button on:click={() => {dashboardController.runIterations(numberOfIterations, $dashboardController?.species, parseInt(speciesSelector.value));}}>Run</button></td>
                    </tr>
                </table>
            {/if}

        </div>




        <div id="batch-dashboard">

            <div id="batch-left">
                <p>Batch programming</p>

                <table bind:this={batchProgrammingTable} border={1}>
                    {#if ($dashboardController != undefined)}
                        {#each batchEntries as be, i}
                        <tr>
                            <td>
                                <BatchEntryRow bind:batchEntry={be} />
                            </td>
                            <td>
                                <button on:click={() => {
                                    batchEntries.splice(i, 1);
                                    batchEntries = batchEntries;
                                }}>x</button>
                            </td>
                        </tr>
                        {/each}
                    {/if}

                    <tr>
                        <td>
                            <button on:click={() => {batchEntries.push(new BatchEntry()); batchEntries = batchEntries;}}>Add entry</button>
                        </td>
                    </tr>
                </table>

            </div>

            <div id="batch-right">
                <p>Batch export</p>

                

            </div>

        </div>

    </div>
</div>



<style>

    h1, h2, h3, h4, h5, h6, p, span, a {
        margin: 0;
        padding: 0;
    }

    span.space {
        width: 100%;
        height: 50px;
    }

    div#wrapper {
        width: 100vw;
        height: 100vh;

        position: absolute;
        left: 0;
        top: 0;

        background: #f5f5f5;
    }

    div#stage-container {
        width: 50vw;
        height: 60vh;

        position: absolute;
        left: 0;
        top: 0;
    }

    div#order-parameter-container {
        width: 50vw;
        height: 60vh;

        position: absolute;
        right: 0;
        top: 0;
    }

    div#controls-container {
        width: 100vw;
        height: 40vh;

        position: absolute;
        left: 0;
        bottom: 0;

        font-family: Helvetica;
    }

    div#species {
        width: 20vw;
        height: 90%;

        position: absolute;
        left: 20px;
        top: 0;
    }

    select#species-select {
        width: 100%;
    }

    div#single-run-dashboard {
        width: 30vw;
        height: 90%;

        position: absolute;
        left: calc(20vw + 20px);
        top: 0;
    }


    p#no-species-selected {
        text-align: center;

        position: relative;
        left: 50%;
        top: 0;

        transform: translateX(-50%);
        -webkit-transform: translateX(-50%);
    }

    p#selected-species {
        text-align: center;

        position: relative;
        left: 50%;
        top: 0;

        transform: translateX(-50%);
        -webkit-transform: translateX(-50%);
    }

    div#single-run-dashboard table {
        width: 80%;

        position: relative;
        left: 50%;
        top: 100px;

        transform: translate(-50%, -50%);
        -webkit-transform: translate(-50%, -50%);
    }






    div#batch-dashboard {
        width: calc(50vw - 20px);
        height: 90%;

        position: absolute;
        left: calc(50vw + 20px);
        top: 0;

        border-left: solid 2px #bbb;
    }

    div#batch-dashboard table {
        margin-top: 20px;

        border: 1px solid #888;
        border-collapse: collapse;
    }

    div#batch-left {
        width: 50%;
        height: 100%;

        position: absolute;
        left: 0;
        top: 0;
    }

    div#batch-right {
        width: 50%;
        height: 100%;

        position: absolute;
        right: 0;
        top: 0;
    }

</style>