




export class BatchExportEntry {


    // A BatchEntry should specify which variable of which chemical, belonging to which species should be varied
    attribute: string;


    constructor() {
        this.attribute = "";
    }

    static withAttribute(attribute: string) {
        const entry = new BatchExportEntry();
        entry.attribute = attribute;
        return entry;
    }

}