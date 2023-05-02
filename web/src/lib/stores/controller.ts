/**
 * The controller store is responsible for all communication between Svelte and Rust through WebAssembly
 */

import { writable } from "svelte/store";
import init, * as wasm from "$lib/wasm/pkg/wasm";

function createControllerStore() {
    const { subscribe, set, update } = writable<wasm.CAGrid2D>();

    // Variable to keep track of initialisation status of wasm
    let wasmInitialised = false;

    // Return the store and all functions to go along with it
    return {
        subscribe,

        /**
         * Method: initialise wasm
         */
        initialise: async () => {
            if (!wasmInitialised) {
                await init();
                wasmInitialised = true;
            }
        },

        /**
         * Method: Create a new empty grid of the specified size
         */
        createEmptyGrid: (size: number) => {
            set(wasm.CAGrid2D.new(size));
        },

        /**
         * Method: randomly spread the specified number of chemicals over the grid
         */
        randomlySpreadChemicals: (chemicals: number) => {
            update(grid => wasm.spread_chemicals_randomly_2d(grid, chemicals));
        },

        /**
         * Method: run one iteration of the algorithm
         */
        runIteration: () => {
            update(grid => wasm.run_iteration(grid));
        }
    }
}

export const controller = createControllerStore();