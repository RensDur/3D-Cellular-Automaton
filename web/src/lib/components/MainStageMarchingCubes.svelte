<script lang="ts">
    import * as THREE from "three";
	import { ColoredBox } from "$lib/classes/ColoredBox";
	import { SliceMovement } from "$lib/data/SliceMovement";
	import { onMount } from "svelte";
    import { OrbitControls } from "three/addons/controls/OrbitControls.js";
    import { GLTFLoader, type GLTF } from 'three/examples/jsm/loaders/GLTFLoader';
    import { DRACOLoader } from "three/examples/jsm/loaders/DRACOLoader";
    import { controller } from "$lib/stores/controller";

    // Exposures
    export let sceneWidth: number | undefined = undefined;
    export let sceneHeight: number | undefined = undefined;

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

    let meshGeometries: any[] = [];
    let meshObjectsBackSide: any[] = [];
    let meshObjectsFrontSide: any[] = [];

    // Colors!
    let chemical_colors = [0xc2532b, 0x4287f5, 0x7c1e79, 0x9862a5, 0xb8d161, 0x5db98d, 0xcc4a4a]

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

        if (sceneWidth == undefined || sceneHeight == undefined) {
            renderer.setSize(window.innerWidth, window.innerHeight);
        } else {
            renderer.setSize(sceneWidth, sceneHeight);
        }

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
        if (sceneWidth == undefined || sceneHeight == undefined) {
            camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
        } else {
            camera = new THREE.PerspectiveCamera(75, sceneWidth / sceneHeight, 0.1, 1000);
        }
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

        function initGltf() {
            // Add the mesh for the Cellullar Automaton
            gltfLoader.load(
                controller.getGltfUrl(),

                function (gltf: GLTF) {
                    console.log("This is the gltf:");
                    console.log(gltf);

                    // Remove existing geometries from the scene, if there are any
                    for (let i = 0; i < meshGeometries.length; i++) {
                        scene.remove(meshObjectsFrontSide[i]);
                        scene.remove(meshObjectsBackSide[i]);
                    }

                    meshGeometries = [];
                    meshObjectsFrontSide = [];
                    meshObjectsBackSide = [];

                    let geometriesToCover = [];

                    if (gltf.scene.children[0].geometry) {
                        geometriesToCover.push(gltf.scene.children[0].geometry);
                    } else {
                        for (let i = 0; i < gltf.scene.children[0].children.length; i++) {
                            geometriesToCover.push(gltf.scene.children[0].children[i].geometry);
                        }
                    }

                    for (let i = 0; i < geometriesToCover.length; i++) {
                        meshGeometries.push(geometriesToCover[i]);
                        meshGeometries[i].computeVertexNormals();
                        meshGeometries[i].translate(-size/2, -size/2, -size/2);

                        // Decide which colour should appear where
                        // The back-side of the mesh is always the chemical that was selected
                        let colorA = chemical_colors[0];
                        let colorB = chemical_colors[1];

                        // If the controller indicates that the current-working-device is the nchem-device
                        if (controller.getWorkingDevice() == "nchem") {
                            // The colors need to be changed.
                            // The selected cell-type is:
                            let selectedChemicalCapture = $controller.nChemChemicalCapture;

                            console.log("The selected cell-type is: " + String(selectedChemicalCapture));

                            // colorA will become the color that corresponds to this cell-type
                            colorA = 0xfccf03;

                            // colorB will become grey
                            colorB = chemical_colors[selectedChemicalCapture];
                        }

                        // Show both sides of the mesh with a different color
                        meshObjectsFrontSide.push(new THREE.Mesh(meshGeometries[i], new THREE.MeshPhongMaterial({color: colorA, side: THREE.FrontSide})));
                        meshObjectsBackSide.push(new THREE.Mesh(meshGeometries[i], new THREE.MeshPhongMaterial({color: colorB, side: THREE.BackSide})));

                        // Show the edges of the mesh
                        // const edges = new THREE.EdgesGeometry(meshGeometries[i]);
                        // meshObjectsFrontSide.push(new THREE.LineSegments(edges, new THREE.LineBasicMaterial({color: 0x333333})));
                        // meshObjectsBackSide.push(new THREE.LineSegments(edges, new THREE.LineBasicMaterial({color: 0x333333})));

                        // Set the shadow casting properties
                        meshObjectsFrontSide[i].castShadow = true;
                        meshObjectsFrontSide[i].receiveShadow = false;

                        meshObjectsBackSide[i].castShadow = true;
                        meshObjectsBackSide[i].receiveShadow = false;

                        scene.add(meshObjectsFrontSide[i]);
                        scene.add(meshObjectsBackSide[i]);
                    }
                },
                
                undefined,

                function(event: ErrorEvent) {
                    console.error(event);
                }
            );
        }

        // Attempt to initialise the gltf immediately
        initGltf();


        function animate() {
            requestAnimationFrame(animate);

            // Update the orbit-controls
            orbitControls.update();

            // Update the number of the slice that's displayed according to scroll
            // We'll only change the slice once the user has selected a different one.
            // If the new slice selection is different from the current slice
            if ($controller != previouslyRenderedGrid) {

                // Update the displayed mesh
                initGltf();

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
        if (sceneWidth == undefined || sceneHeight == undefined) {
            camera.aspect = window.innerWidth / window.innerHeight;
        } else {
            camera.aspect = sceneWidth / sceneHeight;
        }
        
        camera.updateProjectionMatrix();

        // Update the size of the renderer to match the new window-size
        if (sceneWidth == undefined || sceneHeight == undefined) {
            renderer.setSize(window.innerWidth, window.innerHeight);
        } else {
            renderer.setSize(sceneWidth, sceneHeight);
        }
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