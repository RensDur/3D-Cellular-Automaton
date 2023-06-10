/**
 * The controller store is responsible for all communication between Svelte and Rust through WebAssembly
 */

import { DashboardStore } from "$lib/classes/DashboardStore";
import { Species } from "$lib/classes/Species";
import { writable } from "svelte/store";

import { controller } from "$lib/stores/controller";
import { Chemical } from "$lib/classes/Chemical";
import type { BatchEntry } from "$lib/classes/BatchEntry";
import type { BatchExportEntry } from "$lib/classes/BatchExportEntry";

function createDashboardControllerStore() {
    const { subscribe, set, update } = writable<DashboardStore>();

    const serverAddress = "http://localhost:7878";
    const workingAddress = "nchem";

    async function getCurrentMCMeshFromServer() {
        const response = await fetch(serverAddress + "/" + workingAddress + "/get-current-state-triangles", {
            method: "GET"
        });

        const res = await response.json();
        
        return res;
    }

    async function getOrderParameterFromServer() {
        const response = await fetch(serverAddress + "/nchem/get-order-parameter", {
            method: "GET"
        });

        return await response.json();
    }




    async function sendPost(path: string, data?: object) {
        let fields: object = {method: "POST"};

        if (data) {
            fields = {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify(data)
            }
        }

        const response = await fetch(serverAddress + path, fields);
        return await response.text();
    }

    async function sendGet(path: string) {
        const response = await fetch(serverAddress + path, {method: "GET"});
        return await response.text();
    }

    async function sendGetJson(path: string) {
        const response = await fetch(serverAddress + path, {method: "GET"});
        return await response.json();
    }

    async function sendDevicePostWithJson(path: string, data: object) {
        const response = await fetch(serverAddress + "/" + workingAddress + path, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(data)
        });
        return await response.json();
    }
    

    // Return the store and all functions to go along with it
    return {
        subscribe,

        initialise: async () => {
            const store = new DashboardStore();

            // Get the species configuration from the server
            const speciesConfig = await sendGetJson("/nchem/get-species-configuration");

            // Translate this model into the species array
            const species: Species[] = [];

            for (let i = 0; i < speciesConfig.length; i++) {
                const ca = new Chemical(speciesConfig[i].promote.range, speciesConfig[i].promote.influence);
                const cb = new Chemical(speciesConfig[i].demote.range, speciesConfig[i].demote.influence);

                species.push(new Species(ca, cb));
            }

            store.species = species;

            update(_ => store);
        },

        runIterations: async (iter: number, species: Species[], selectedSpecies: number) => {

            // Get the automaton size from the server
            const size: number = await sendGetJson("/general/get-automaton-size");

            // 1. Update the species-specification on the server
            await sendPost("/nchem/set-species-configuration", {species});

            // 2. Randomly spread as many chemicals as there are species + 1
            await sendPost("/general/spread-chemicals-randomly", {"chemicals": species.length + 1});

            // 3. Run the specified number of iterations
            const duration = await sendDevicePostWithJson("/run-iteration", {num_iterations: iter});

            // 4. Update the number of iterations run
            const iterations = await sendGet("/nchem/get-iterations");

            // 5. Update the order parameter
            const orderParameter = await getOrderParameterFromServer();

            // 6. Update the 3D graph
            await sendDevicePostWithJson("/set-chemical-capture", {chemical_capture: selectedSpecies});
            const marchingCubesGltf = await getCurrentMCMeshFromServer();

            // Update the general controller to make other components work with this new data
            controller.pushDashboardUpdate(size, iterations, orderParameter, marchingCubesGltf, selectedSpecies);

            // Return the average duration per iteration for this round of simulation
            return duration.duration / iter;
        },

        requestMeshForSpecies: async (selectedSpecies: number) => {

            // Get the automaton size from the server
            const size: number = await sendGetJson("/general/get-automaton-size");

            // 4. Update the number of iterations run
            const iterations = await sendGet("/nchem/get-iterations");

            // 5. Update the order parameter
            const orderParameter = await getOrderParameterFromServer();

            // 6. Update the 3D graph
            await sendDevicePostWithJson("/set-chemical-capture", {chemical_capture: selectedSpecies});
            const marchingCubesGltf = await getCurrentMCMeshFromServer();

            // Update the general controller to make other components work with this new data
            controller.pushDashboardUpdate(size, iterations, orderParameter, marchingCubesGltf, selectedSpecies);

        },


        runBatchExperiment: async (species: Species[], entries: BatchEntry[], exportEntries: BatchExportEntry[], iterations: number, file_name: string, csvSettingFloatingPoint: string) => {

            // Setup the experiment in the correct format
            const data = {
                "entries": entries,
                "export_entries": exportEntries,
                "iterations": iterations,
                "file_name": file_name,
                "floating_point": csvSettingFloatingPoint
            };

            // 1. Update the species-specification on the server
            await sendPost("/nchem/set-species-configuration", {species});

            // 2. Signal the server to run this batch 
            await sendPost("/batch/run-experiment", data);

        }
        
    };
}

export const dashboardController = createDashboardControllerStore();