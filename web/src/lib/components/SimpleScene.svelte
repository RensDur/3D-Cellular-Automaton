<script lang="ts">
	import { onMount } from 'svelte';
    import * as THREE from 'three';
    import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
    import { ColoredBox } from '$lib/classes/ColoredBox';

    enum SliceMovement {
        MoveSlice,
        MoveBoundingBox
    }

    let sliceMovement = SliceMovement.MoveSlice;

    let scrollPosition = 0;
    let displayedSlice: number = 0;
    let newDisplayedSlice: number = 0;
    const size = 20;

    let renderedBoxes: Array<ColoredBox> = [];

    function setupScene() {

        // Setup a new three.js scene
        const scene = new THREE.Scene();
        scene.background = new THREE.Color(0xf5f5f5);

        // Setup the WebGL renderer
        const renderer = new THREE.WebGLRenderer({antialias: true});
        renderer.setSize(window.innerWidth, window.innerHeight);

        // Add the domElement from this renderer to the div#container
        document.getElementById("container")?.appendChild(renderer.domElement);

        // Specify the camera properties
        const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
        camera.position.set(size, size, size);

        // Specify the orbit-controls
        const orbitControls = new OrbitControls(camera, renderer.domElement);

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
            if (newDisplayedSlice != displayedSlice) {
                displayedSlice = newDisplayedSlice;

                // Add new boxes to the scene
                showSlice(displayedSlice);

            }

            renderer.render(scene, camera);
        }

        animate();
    }

    onMount(setupScene);

    function handleMouseWheelEvent(e: WheelEvent) {
        scrollPosition += (e.deltaY * size)/1000;
        
        if (scrollPosition >= size-1) {
            scrollPosition = size-1;
        } else if (scrollPosition <= 0) {
            scrollPosition = 0;
        }

        newDisplayedSlice = Math.round(scrollPosition);
        
    }

</script>


<div id="container" on:wheel={handleMouseWheelEvent}>

</div>


<style>

    div#container {
        width: 100%;
        height: 100%;

        margin: 0;
        padding: 0;

        background: green;
    }

</style>