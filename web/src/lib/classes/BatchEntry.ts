




export class BatchEntry {


    // A BatchEntry should specify which variable of which chemical, belonging to which species should be varied
    species: number;
    chemical: string;
    variable: string;

    min: number;
    max: number;
    step: number;


    constructor() {
        this.species = 0;
        this.chemical = "";
        this.variable = "";
        this.min = 0;
        this.max = 1;
        this.step = 0.01;
    }


}