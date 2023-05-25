import * as THREE from "three";
import { type GLTF } from 'three/examples/jsm/loaders/GLTFLoader';


export class ChunkGeometry {

    initialised: boolean;
    geometry: THREE.BufferGeometry;
    frontMesh: THREE.Mesh;
    backMesh: THREE.Mesh;

    constructor() {
        this.initialised = false;
        this.geometry = new THREE.BoxGeometry();
        this.frontMesh = new THREE.Mesh();
        this.backMesh = new THREE.Mesh();
    }

    insertInScene(scene: THREE.Scene) {
        if (this.initialised) {
            scene.add(this.frontMesh);
            scene.add(this.backMesh);
        }
    }

    init(gltf: GLTF, scene: THREE.Scene) {
        console.log("This is the gltf:");
        console.log(gltf);

        this.geometry = gltf.scene.children[0].geometry;
        this.geometry.computeVertexNormals();

        this.backMesh = new THREE.Mesh(this.geometry, new THREE.MeshPhongMaterial({color: "#c2532b", side: THREE.BackSide}));
        this.frontMesh = new THREE.Mesh(this.geometry, new THREE.MeshPhongMaterial({color: "#e3a474", side: THREE.FrontSide}));

        // Set the shadow casting properties
        this.backMesh.castShadow = true;
        this.backMesh.receiveShadow = false;

        this.frontMesh.castShadow = true;
        this.frontMesh.receiveShadow = false;

        this.initialised = true;

        this.insertInScene(scene);
    }

    update(gltf: GLTF, scene: THREE.Scene) {

        console.log("This is the gltf:");
        console.log(gltf);

        if (this.initialised) {
            this.geometry = gltf.scene.children[0].geometry;
            this.geometry.computeVertexNormals();
    
            this.backMesh.geometry = this.geometry;
            this.frontMesh.geometry = this.geometry;
        } else {
            this.init(gltf, scene);
        }

    }

    translateNeg(x: number, y: number, z: number) {
        if (this.initialised) {
            this.geometry.translate(-x, -y, -z);
        }
    }

}