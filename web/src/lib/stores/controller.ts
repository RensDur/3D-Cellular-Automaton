/**
 * The controller store is responsible for all communication between Svelte and Rust through WebAssembly
 */

import { Grid3D } from "$lib/classes/Grid3D";
import { writable } from "svelte/store";

function createControllerStore() {
    const { subscribe, set, update } = writable<Grid3D>();

    const serverAddress = "http://localhost:7878";
    let workingAddress = "cpu";

    async function getCurrentGridFromServer() {
        const response = await fetch(serverAddress + "/" + workingAddress + "/get-current-state", {
            method: "GET"
        });
    
        const result = await response.json();
        
        if (workingAddress == "cpu") {
            return result.curr_generation.data;
        } else {
            return result.grid;
        }
    }

    async function getCurrentMCMeshFromServer() {
        const response = await fetch(serverAddress + "/" + workingAddress + "/get-current-state-triangles", {
            method: "GET"
        });

        const res = await response.json();
        
        return res;
    }

    async function updateStore() {
        const state = await getCurrentGridFromServer();
        const grid = Grid3D.from(state.length, state);

        // Get the MC Mesh from the server
        const mcGltf = await getCurrentMCMeshFromServer();
        grid.setMarchingCubesGltf(mcGltf);

        // Update both the cpu and gpu number of iterations
        grid.cpuIterations = await sendGet("/cpu/get-iterations");
        grid.gpuIterations = await sendGet("/gpu/get-iterations");

        console.log("The current grid state was requested from the server. Response:");
        console.log(grid);

        update(_ => grid);
    }

    async function sendDevicePost(path: string) {
        const response = await fetch(serverAddress + "/" + workingAddress + path, {method: "POST"});
        return await response.text();
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

    // Return the store and all functions to go along with it
    return {
        subscribe,

        getWorkingDevice: () => {
            return workingAddress;
        },

        getGltfUrl: () => {
            return serverAddress + "/" + workingAddress + "/get-current-state-triangles"
        },

        /**
         * Method: initialise wasm
         */
        initialise: async () => {
            await updateStore();
        },

        selectSimulationDevice: async (device: string) => {
            workingAddress = device;
            await updateStore();
        },

        clearGrid: async () => {
            await sendDevicePost("/clear-all-voxels")
            await updateStore();
        },

        /**
         * Method: randomly spread the specified number of chemicals over the grid
         */
        randomlySpreadChemicals: async (chemicals: number) => {
            await sendPost("/general/spread-chemicals-randomly", {chemicals});
            await updateStore();
        },

        /**
         * Method: run one iteration of the algorithm
         */
        runIteration: async () => {
            const duration = await sendDevicePostWithJson("/run-iteration", {num_iterations: 1});
            await updateStore();

            console.log("Calculated 1 iteration in " + String(duration.duration) + " seconds")
        },

        /**
         * Method: run twenty iterations of the algorithm
         */
        run5Iterations: async () => {
            const duration = await sendDevicePostWithJson("/run-iteration", {num_iterations: 5});
            await updateStore();

            console.log("Calculated 5 iterations in " + String(duration.duration) + " seconds")
        },

        /**
         * Method: perform comparison benchmarks on the server
         */
        compareCPUvsGPUNow: async () => {
            return await sendPost("/benchmarks/compare-cpu-gpu");
        },

        compareCPUvsGPUAfterCatchUp: async () => {
            const result = await sendPost("/benchmarks/compare-cpu-gpu-catch-up");
            await updateStore();
            return result;
        },

        benchmarkGPUShaderIncrement: async () => {
            const result = await sendPost("/benchmarks/gpu-shader-increment");
            await updateStore();
            return result;
        },

        generatePatch: async () => {
            const result = await sendPost("/general/create-activator-patch");
            await updateStore();
            return result;
        }
    }
}

export const controller = createControllerStore();