import type { Species } from "./Species";

export class DashboardStore {

    species: Species[];

    constructor() {
        this.species = [];
    }

    insertSpecies(species: Species) {
        this.species.push(species);
    }

    removeSpecies(index: number) {
        this.species.splice(index, 1);
    }

}