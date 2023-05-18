export class Grid3D {

    // Raw data
    size: number;
    data: number[][][];

    // Marching Cubes mesh data
    // This representation matches that of THREE.js, where three subsequent triangles form one face.
    mcPositions: number[][];

    cpuIterations: string;
    gpuIterations: string;

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

        this.mcPositions = [];

        this.cpuIterations = "0";
        this.gpuIterations = "0";
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

    // Getters and setters for the Marching Cubes mesh
    setMCPositions(positions: number[][]) {
        this.mcPositions = positions;
    }

    getMCPositions() {
        return this.mcPositions;
    }

    exportMCMeshPositions() {
        const result: number[] = [];

        for (const vec of this.mcPositions) {
            result.push(vec[0]);
            result.push(vec[1]);
            result.push(vec[2]);
        }

        return result;
    }

}