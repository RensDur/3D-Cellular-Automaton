/**
 * The controller store is responsible for all communication between Svelte and Rust through WebAssembly
 */

import { DashboardStore } from "$lib/classes/DashboardStore";
import { writable } from "svelte/store";

function createDashboardControllerStore() {
    const { subscribe, set, update } = writable<DashboardStore>();

    

    // Return the store and all functions to go along with it
    return {
        subscribe,

        initialise: () => {
            const store = new DashboardStore();

            update(_ => store);
        }
        
    };
}

export const dashboardController = createDashboardControllerStore();