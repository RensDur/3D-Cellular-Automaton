export class Grid3D {

    // Raw data
    size: number;
    data: number[][][];

    // Marching Cubes mesh data
    // This representation matches that of THREE.js, where three subsequent triangles form one face.
    marchingCubesGltf: ArrayBuffer;

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

        this.marchingCubesGltf = new ArrayBuffer(0);

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
    setMarchingCubesGltf(gltf: number[]) {
        const buffer = new ArrayBuffer(gltf.length * 4);
        const array = new Uint32Array(buffer);
        for (let i = 0; i < gltf.length; i++) {
            array[i] = gltf[i];
        }
        this.marchingCubesGltf = buffer;
    }

    getMarchingCubesGltf() {
        return this.marchingCubesGltf;
    }

}