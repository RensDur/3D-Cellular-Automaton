/**
 * The controller store is responsible for all communication between Svelte and Rust through WebAssembly
 */

import { writable } from "svelte/store";
import init, * as wasm from "$lib/wasm/pkg/wasm";

function createControllerStore() {
    const { subscribe, set, update } = writable<wasm.CellularAutomaton3D>();

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

                set(wasm.CellularAutomaton3D.new(30, 2, 1.0, 4, -0.2));
            }
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