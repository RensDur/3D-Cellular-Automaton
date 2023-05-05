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
        return result.curr_generation;
    }

    async function updateStore() {
        const state = await getCurrentGridFromServer();
        update(_ => Grid3D.from(state.size, state.data));
    }

    // Return the store and all functions to go along with it
    return {
        subscribe,

        /**
         * Method: initialise wasm
         */
        initialise: async (size: number, dc_range: number, dc_influence: number, uc_range: number, uc_influence: number) => {
            const response = await fetch("http://localhost:7878/initialise", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({size, dc_range, dc_influence, uc_range, uc_influence})
            });
            await response.text();

            await updateStore();
        },

        clearGrid: () => {
            update(ca => ca.reset_all_voxels());
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
        },

        /**
         * Method: randomly spread the specified number of chemicals over the grid
         */
        randomlySpreadChemicals: (chemicals: number) => {
            update(ca => ca.spread_chemicals_randomly(chemicals));
        },

        /**
         * Method: run one iteration of the algorithm
         */
        runIteration: () => {
            update(ca => ca.run_iteration());
        }
    }
}

export const controller = createControllerStore();