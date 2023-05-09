/**
 * The controller store is responsible for all communication between Svelte and Rust through WebAssembly
 */

import { Grid3D } from "$lib/classes/Grid3D";
import { writable } from "svelte/store";

function createControllerStore() {
    const { subscribe, set, update } = writable<Grid3D>();

    async function getCurrentGridFromServer() {
        const response = await fetch("http://localhost:7878/get-current-state", {
            method: "GET"
        });
    
        const result = await response.json();
        return result.grid;
    }

    async function updateStore() {
        const state = await getCurrentGridFromServer();
        const grid = Grid3D.from(state.length, state);

        console.log("The current grid state was requested from the server. Response:");
        console.log(grid);

        update(_ => grid);
    }

    async function sendPost(path: string) {
        const response = await fetch("http://localhost:7878" + path, {method: "POST"});
        await response.text();
    }

    async function sendPostWithJson(path: string, data: object) {
        const response = await fetch("http://localhost:7878" + path, {
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

        /**
         * Method: initialise wasm
         */
        initialise: async (size: number, dc_range: number, dc_influence: number, uc_range: number, uc_influence: number) => {
            await sendPostWithJson("/initialise", {size, dc_range, dc_influence, uc_range, uc_influence});
            await updateStore();
        },

        clearGrid: async () => {
            await sendPost("/clear-all-voxels")
            await updateStore();
        },

        /**
         * Method: randomly spread the specified number of chemicals over the grid
         */
        randomlySpreadChemicals: async (chemicals: number) => {
            await sendPostWithJson("/spread-chemicals-randomly", {chemicals});
            await updateStore();
        },

        /**
         * Method: run one iteration of the algorithm
         */
        runIteration: async () => {
            const duration = await sendPostWithJson("/run-iteration", {num_iterations: 1});
            await updateStore();

            console.log("Calculated 1 iteration in " + String(duration.duration) + " seconds")
        },

        /**
         * Method: run twenty iterations of the algorithm
         */
        run5Iterations: async () => {
            const duration = await sendPostWithJson("/run-iteration", {num_iterations: 5});
            await updateStore();

            console.log("Calculated 5 iterations in " + String(duration.duration) + " seconds")
        },

        /**
         * Method: update dc and uc parameters
         */
        updateDCRange: (dc_range: number) => {
            update(ca => ca.set_dc_range(dc_range));
        },
        updateDCInfluence: (dc_influence: number) => {
            update(ca => ca.set_dc_influence(dc_influence));
        },
        updateUCRange: (uc_range: number) => {
            update(ca => ca.set_uc_range(uc_range));
        },
        updateUCInfluence: (uc_influence: number) => {
            update(ca => ca.set_uc_influence(uc_influence));
        }
    }
}

export const controller = createControllerStore();