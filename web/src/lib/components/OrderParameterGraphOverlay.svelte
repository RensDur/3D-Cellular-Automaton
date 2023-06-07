<script lang="ts">
	import { onMount } from 'svelte';
    import ControlCenter from './ControlCenter.svelte';

    import { tweened } from 'svelte/motion';
    import { cubicInOut, quadInOut } from 'svelte/easing';
	import OrderParameterGraph from './OrderParameterGraph.svelte';

    // Window width
    let windowWidth = 0;
    let windowHeight = 0;

    // Canvas size
    let canvasWidth = 100;
    let canvasHeight = 100;

    // State boolean
    let csShown = false;

    // Default container size-values
    const hiddenSize = 30;
    const hiddenHoverSize = 40;

    const animationProperties = {
        duration: 500,
        easing: cubicInOut
    }

    const csWidth = tweened(hiddenSize, animationProperties);


    // Event handlers
    function hideControlCenter() {
        csWidth.set(hiddenSize);
        csShown = false;
    }

    function openControlCenter() {
        csWidth.set(600);
        csShown = true;
    }

    function toggleControlCenter() {
        if (csShown) {
            hideControlCenter();
        } else {
            openControlCenter();
        }
    }

    function hoverInControlCenter() {
        if (!csShown) {
            csWidth.set(hiddenHoverSize);
        }
    }

    function hoverOutControlCenter() {
        if (!csShown) {
            hideControlCenter();
        }
    }

    onMount(() => {
        toggleControlCenter();
    })

</script>


<!-- Container for the control center -->
<div    id="container"
        style="width: {$csWidth}px;"
        on:mouseenter={hoverInControlCenter}
        on:mouseleave={hoverOutControlCenter}
        on:mousedown={openControlCenter}>

        <div id="order-parameter-graph-wrapper" bind:clientWidth={canvasWidth} bind:clientHeight={canvasHeight}>
            <OrderParameterGraph bind:windowWidth={canvasWidth} bind:windowHeight={canvasHeight}/>
        </div>

        <button id="toggleButton"
                on:click={toggleControlCenter}>
        </button>

</div>

<svelte:window bind:innerWidth={windowWidth} bind:innerHeight={windowHeight} />


<style>

    :root {
        --border-radius: 1.5rem;
    }

    div#container {
        height: 400px;

        position: absolute;
        right: 0;
        top: calc(100vh - 430px);

        backdrop-filter: blur(20px);

        background: #e5e5e0aa;
        box-shadow: 0 0 8px 2px #aaaaaa99;

        box-sizing: border-box;
        border-left: 1px solid #999;
        border-top: 1px solid #999;
        border-bottom: 1px solid #999;
        border-top-left-radius: 25px;
        border-bottom-left-radius: 25px;

        overflow: hidden;
    }

    button#toggleButton {
        -webkit-appearance: none;
        appearance: none;

        width: 10px;
        height: 3rem;

        position: absolute;
        left: 5px;
        top: calc(50% - 1.5rem);

        border: none;
        border-radius: var(--border-radius);

        background: #959590;

        transition: width 0.1s ease-in-out;
        -webkit-transition: width 0.1s ease-in-out;
    }

    div#container:hover button#toggleButton {
        width: 15px;
    }

    div#order-parameter-graph-wrapper {
        width: calc(100% - 60px);
        min-width: 200px;
        height: calc(100% - 60px);
        max-height: calc(100% - 60px);

        position: absolute;
        right: 30px;
        top: 30px;
    }

</style>