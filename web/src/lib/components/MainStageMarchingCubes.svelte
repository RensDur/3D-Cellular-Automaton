<script lang="ts">
    import * as THREE from "three";
	import { ColoredBox } from "$lib/classes/ColoredBox";
	import { SliceMovement } from "$lib/data/SliceMovement";
	import { onMount } from "svelte";
    import { OrbitControls } from "three/addons/controls/OrbitControls.js";
    import { controller } from "$lib/stores/controller";

    // DOM bindings
    let containerDiv: HTMLDivElement;

    // THREE.js elements
    let scene: THREE.Scene;
    let renderer: THREE.WebGLRenderer;
    let ambientLight: THREE.AmbientLight;
    let pointLight1: THREE.PointLight;
    let pointLight2: THREE.PointLight;
    let camera: THREE.PerspectiveCamera;
    let orbitControls: OrbitControls;

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

        // Add the mesh for the Cellullar Automaton
        let meshGeometry = new THREE.BufferGeometry();
        meshGeometry.setAttribute("position", new THREE.Float32BufferAttribute($controller.exportMCMeshPositions(), 3));
        meshGeometry.computeVertexNormals();
        meshGeometry.translate(-size/2, -size/2, -size/2);

        const meshObjectBackSide = new THREE.Mesh(meshGeometry, new THREE.MeshPhongMaterial({color: "#c2532b", side: THREE.BackSide}));
        const meshObjectFrontSide = new THREE.Mesh(meshGeometry, new THREE.MeshPhongMaterial({color: "#e3a474", side: THREE.FrontSide}));

        // Set the shadow casting properties
        meshObjectBackSide.castShadow = true;
        meshObjectBackSide.receiveShadow = false;

        meshObjectFrontSide.castShadow = true;
        meshObjectFrontSide.receiveShadow = false;

        scene.add(meshObjectBackSide);
        scene.add(meshObjectFrontSide);


        function updateMeshObject() {
            meshGeometry.setAttribute("position", new THREE.Float32BufferAttribute($controller.exportMCMeshPositions(), 3));
            meshGeometry.computeVertexNormals();
            meshGeometry.translate(-size/2, -size/2, -size/2);

            console.log("meshGeometry: ");
            console.log(meshGeometry);
        }


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