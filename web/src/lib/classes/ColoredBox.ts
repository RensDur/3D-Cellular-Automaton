import * as THREE from 'three';

export class ColoredBox {

    // Constants
    boxSize = 1;

    // A colored box has a geometrey, material and reference to the 3D mesh
    geometry: THREE.BoxGeometry;
    material: THREE.MeshBasicMaterial;
    mesh: THREE.Mesh;

    // Constructor
    constructor(x: number, y: number, z: number, size: number, translateZ: boolean, color: THREE.Color) {

        this.geometry = new THREE.BoxGeometry(this.boxSize, this.boxSize, this.boxSize);
        this.geometry.translate(-size/2 + this.boxSize/2, -size/2 + this.boxSize/2, translateZ ? -size/2 + this.boxSize/2 : this.boxSize/2);

        this.material = new THREE.MeshBasicMaterial();
        this.material.color = color;

        this.mesh = new THREE.Mesh(this.geometry, this.material);
        this.mesh.position.set(x, y, z);
    }

    // Method: get current position
    getX(): number {
        return this.mesh.position.x;
    }
    getY(): number {
        return this.mesh.position.y;
    }
    getZ(): number {
        return this.mesh.position.z;
    }

    // Method: change the position of this cube
    update(z: number, color: THREE.Color) {
        this.material.color = color;
        this.mesh.position.setZ(z);
    }

    // Method: add this cube to the provided scene
    addToScene(scene: THREE.Scene) {
        scene.add(this.mesh);
    }

    // Method: remove this cube from the provided scene, if possible
    removeFromScene(scene: THREE.Scene) {
        scene.remove(this.mesh);
    }

    // Method: dispose
    dispose() {
        this.geometry.dispose();
        this.material.dispose();
    }

}