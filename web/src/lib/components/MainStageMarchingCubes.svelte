<script lang="ts">
    import * as THREE from "three";
	import { ColoredBox } from "$lib/classes/ColoredBox";
	import { SliceMovement } from "$lib/data/SliceMovement";
	import { onMount } from "svelte";
    import { OrbitControls } from "three/addons/controls/OrbitControls.js";
    import { GLTFLoader, type GLTF } from 'three/examples/jsm/loaders/GLTFLoader';
    import { DRACOLoader } from "three/examples/jsm/loaders/DRACOLoader";
    import { controller } from "$lib/stores/controller";
	import { ChunkGeometry } from "$lib/classes/ChunkGeometry";

    // CONSTANTS
    const chunkSplit = 2;

    // GLTF Chunks
    let gltfChunks: ChunkGeometry[][][] = [];

    // Initialise the gltf chunks
    for (let x = 0; x < chunkSplit; x++) {
        gltfChunks.push([]);
        for (let y = 0; y < chunkSplit; y++) {
            gltfChunks[x].push([]);
            for (let z = 0; z < chunkSplit; z++) {
                gltfChunks[x][y].push(new ChunkGeometry());
            }
        }
    }

    // DOM bindings
    let containerDiv: HTMLDivElement;

    // THREE.js elements
    let gltfLoader = new GLTFLoader();

    // Optional: Provide a DRACOLoader instance to decode compressed mesh data
    const dracoLoader = new DRACOLoader();
    dracoLoader.setDecoderPath( '/examples/jsm/libs/draco/' );
    gltfLoader.setDRACOLoader( dracoLoader );

    let scene: THREE.Scene;
    let renderer: THREE.WebGLRenderer;
    let ambientLight: THREE.AmbientLight;
    let pointLight1: THREE.PointLight;
    let pointLight2: THREE.PointLight;
    let camera: THREE.PerspectiveCamera;
    let orbitControls: OrbitControls;

    let meshGeometry: any;
    let meshObjectBackSide: any;
    let meshObjectFrontSide: any;

    // THREE.js behaviour variables
    let size: number = 20;
    let previouslyRenderedGrid: any;

    // THREE.js setup
    function setupScene() {

        size = $controller.size;

        // Setup the three.js scene
        scene = new THREE.Scene();
        scene.background = new THREE.Color(0xf5f5f5);

        // Setup the WebGL renderer
        renderer = new THREE.WebGLRenderer({antialias: true});
        renderer.setSize(window.innerWidth, window.innerHeight);
        renderer.shadowMap.enabled = true;
        renderer.shadowMap.type = THREE.BasicShadowMap;

        ambientLight = new THREE.AmbientLight(0xffffff, 0.3);
        scene.add(ambientLight);

        pointLight1 = new THREE.PointLight(0xffffff, 1);
        pointLight1.position.set(2*size, 2*size, 2*size);
        pointLight1.castShadow = true;
        scene.add(pointLight1);

        pointLight2 = new THREE.PointLight(0xffffff, 1);
        pointLight2.position.set(-2*size, -2*size, -2*size);
        pointLight2.castShadow = true;
        scene.add(pointLight2);

        // Add the dom-element of the renderer to the container
        containerDiv.appendChild(renderer.domElement);

        // Specify the camera properties
        camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
        camera.position.set(size, size, size);
        
        // Specify the orbit-controls
        orbitControls = new OrbitControls(camera, renderer.domElement);
        
        // Disable zoom through scrolling
        orbitControls.enableZoom = false;

        // Initial update to the controls
        orbitControls.update();

        //------

        // Add the outline cube
        const outlineGeometry = new THREE.BoxGeometry(size, size, size);
        const edgesGeometry = new THREE.EdgesGeometry(outlineGeometry);
        const outlineMaterial = new THREE.LineBasicMaterial({color: 0x111111});
        const outline = new THREE.LineSegments(edgesGeometry, outlineMaterial);

        scene.add(outline);

        function updateMeshObject() {

            const translateStep = size / chunkSplit;

            for (let x = 0; x < chunkSplit; x++) {
                for (let y = 0; y < chunkSplit; y++) {
                    for (let z = 0; z < chunkSplit; z++) {
                        // Use the gltf-loader to get gltf data from the server
                        gltfLoader.load(
                            controller.getGltfChunkUrl(chunkSplit, [x, y, z]),
                            function (gltf) {
                                gltfChunks[x][y][z].update(gltf, scene);
                                gltfChunks[x][y][z].translateNeg(x * translateStep, y * translateStep, z * translateStep);
                            }
                        )
                    }
                }
            }
            
        }

        // Attempt to update the mesh immediately
        updateMeshObject();


        function animate() {
            requestAnimationFrame(animate);

            // Update the orbit-controls
            orbitControls.update();

            // Update the number of the slice that's displayed according to scroll
            // We'll only change the slice once the user has selected a different one.
            // If the new slice selection is different from the current slice
            if ($controller != previouslyRenderedGrid) {

                // Update the displayed mesh
                updateMeshObject();

                // Register the current controller state
                previouslyRenderedGrid = $controller;

                console.log("Updating!");
            }

            renderer.render(scene, camera);
        }

        animate();
    }

    function handleWindowResize(e: Event) {
        // Update the camera aspect-ratio
        camera.aspect = window.innerWidth / window.innerHeight;
        camera.updateProjectionMatrix();

        // Update the size of the renderer to match the new window-size
        renderer.setSize(window.innerWidth, window.innerHeight);
    }


    // Svelte page-mount
    onMount(async () => {
        await controller.initialise();
        setupScene();
    })

</script>

<!-- Capture wheel and resize events -->
<svelte:window on:resize={handleWindowResize}></svelte:window>

<!-- The container for the THREE.js canvas -->
<div id="container" bind:this={containerDiv}></div>


<style>

</style>