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
    let camera: THREE.PerspectiveCamera;
    let orbitControls: OrbitControls;

    // THREE.js behaviour variables
    let size: number = 20;
    let sliceMovement = SliceMovement.MoveSlice;
    let scrollPosition = 0;
    let displayedSlice: number = 0;
    let newDisplayedSlice: number = 0;
    let renderedBoxes: Array<ColoredBox> = [];
    let previouslyRenderedGrid: any;

    // THREE.js setup
    function setupScene() {

        size = $controller.size;

        // Setup the three.js scene
        scene = new THREE.Scene();
        scene.background = new THREE.Color(0xf5f5f3);

        // Setup the WebGL renderer
        renderer = new THREE.WebGLRenderer({antialias: true});
        renderer.setSize(window.innerWidth, window.innerHeight);

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
        if (sliceMovement == SliceMovement.MoveBoundingBox) {
            outlineGeometry.translate(0, 0, size/2);
        }
        const edgesGeometry = new THREE.EdgesGeometry(outlineGeometry);
        const outlineMaterial = new THREE.LineBasicMaterial({color: 0x111111});
        const outline = new THREE.LineSegments(edgesGeometry, outlineMaterial);

        scene.add(outline);

        // Create a slice of cubes
        for (let x = 0; x < size; x++) {
            for (let y = 0; y < size; y++) {

                const coloredBox = new ColoredBox(x, y, displayedSlice, size, sliceMovement == SliceMovement.MoveSlice, new THREE.Color(x / size, 0.5, displayedSlice / size));
                coloredBox.addToScene(scene);

                renderedBoxes.push(coloredBox);

            }
        }

        function showSlice(z: number = 0) {
            // For every box in the registry, set the z-position and calculate the new color

            for (let box of renderedBoxes) {
                let color = new THREE.Color(box.getX() / size, 0.5, z / size);

                if ($controller) {
                    let chemical = $controller.get(box.getX(), box.getY(), z);
                    color = new THREE.Color(0xc2532b);

                    if (chemical == 1) {
                        color = new THREE.Color(0xe3a474);
                    }
                }
                
                if (sliceMovement == SliceMovement.MoveSlice) {
                    box.update(z, color);
                } else if (sliceMovement == SliceMovement.MoveBoundingBox) {
                    box.update(0, color);
                }
            }

            if (sliceMovement == SliceMovement.MoveBoundingBox) {
                outline.position.setZ(-z);
            }
        }

        // Initially, place the slice at z=0
        showSlice(displayedSlice);


        function animate() {
            requestAnimationFrame(animate);

            // Update the orbit-controls
            orbitControls.update();

            // Update the number of the slice that's displayed according to scroll
            // We'll only change the slice once the user has selected a different one.
            // If the new slice selection is different from the current slice
            if (newDisplayedSlice != displayedSlice || $controller != previouslyRenderedGrid) {

                displayedSlice = newDisplayedSlice;

                // Add new boxes to the scene
                showSlice(displayedSlice);

                // Register the current controller state
                previouslyRenderedGrid = $controller;

                console.log("Updating!");
            }

            renderer.render(scene, camera);
        }

        animate();
    }


    // Event handlers
    function handleScroll(e: WheelEvent) {
        scrollPosition += (e.deltaY * size)/1000;
        
        if (scrollPosition >= size-1) {
            scrollPosition = size-1;
        } else if (scrollPosition <= 0) {
            scrollPosition = 0;
        }

        newDisplayedSlice = Math.round(scrollPosition);
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
<svelte:window on:wheel={handleScroll} on:resize={handleWindowResize}></svelte:window>

<!-- The container for the THREE.js canvas -->
<div id="container" bind:this={containerDiv}></div>


<style>

</style>