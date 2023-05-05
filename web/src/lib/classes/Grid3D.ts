export class Grid3D {

    size: number;
    data: number[][][];

    // Constructor
    constructor(size: number) {
        this.size = size;

        this.data = [];

        // Initialise all values in the array to zero
        for (let x = 0; x < size; x++) {
            const xarr: number[][] = [];
            for (let y = 0; y < size; y++) {
                const yarr: number[] = []
                for (let z = 0; z < size; z++) {
                    yarr.push(0);
                }
                xarr.push(yarr);
            }
            this.data.push(xarr);
        }
    }

    static from(size: number, data: number[][][]): Grid3D {
        const grid = new Grid3D(size);
        grid.data = data;

        return grid;
    }

    get(x: number, y: number, z: number): number {
        return this.data[x][y][z];
    }

    set(x: number, y: number, z: number, val: number) {
        this.data[x][y][z] = val;
    }

    getData() {
        return this.data;
    }

    setData(data: number[][][]) {
        this.data = data;
    }

}