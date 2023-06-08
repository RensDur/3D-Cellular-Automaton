import type { Chemical } from "./Chemical";

export class Species {
    chemicalA: Chemical;
    chemicalB: Chemical;

    constructor(ca: Chemical, cb: Chemical) {
        this.chemicalA = ca;
        this.chemicalB = cb;
    }
}

