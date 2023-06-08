<script lang="ts">
	import { Chemical } from "$lib/classes/Chemical";
    import { Species } from "$lib/classes/Species";


    import MainStageMarchingCubes from "$lib/components/MainStageMarchingCubes.svelte";
    import OrderParameterGraph from "$lib/components/OrderParameterGraph.svelte";
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



    // State keeping
    let selectedSpecies: Species | undefined = undefined;
    let numberOfIterations: number = 10;



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
                        <td><button on:click={() => {dashboardController.runIterations(numberOfIterations, $dashboardController?.species);}}>Run</button></td>
                    </tr>
                </table>
            {/if}

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
        height: 50vh;

        position: absolute;
        left: 0;
        top: 0;
    }

    div#order-parameter-container {
        width: 50vw;
        height: 50vh;

        position: absolute;
        right: 0;
        top: 0;
    }

    div#controls-container {
        width: 100vw;
        height: 50vh;

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

        transform: translate(-50%, -50%);
        -webkit-transform: translate(-50%, -50%);
    }

    p#selected-species {
        text-align: center;

        position: relative;
        left: 50%;
        top: 0;

        transform: translate(-50%, -50%);
        -webkit-transform: translate(-50%, -50%);
    }

    div#single-run-dashboard table {
        width: 80%;

        position: relative;
        left: 50%;
        top: 100px;

        transform: translate(-50%, -50%);
        -webkit-transform: translate(-50%, -50%);
    }

</style>