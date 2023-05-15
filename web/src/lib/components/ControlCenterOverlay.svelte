<script lang="ts">
	import { onMount } from 'svelte';
    import ControlCenter from './ControlCenter.svelte';

    import { tweened } from 'svelte/motion';
    import { quadInOut } from 'svelte/easing';

    // State boolean
    let csShown = false;

    // Default container size-values
    const hiddenSize = 30;
    const hiddenHoverSize = 40;

    const animationProperties = {
        duration: 300,
        easing: quadInOut
    }

    const csWidth = tweened(hiddenSize, animationProperties);


    // Event handlers
    function hideControlCenter() {
        csWidth.set(hiddenSize);
        csShown = false;
    }

    function openControlCenter() {
        csWidth.set(420);
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

        <div id="control-center-wrapper">
            <ControlCenter/>
        </div>

        <button id="toggleButton"
                on:click={toggleControlCenter}>
        </button>

</div>


<style>

    :root {
        --border-radius: 1.5rem;
    }

    div#container {
        height: calc(100vh - 60px);

        position: absolute;
        left: 0;
        top: 30px;

        backdrop-filter: blur(12px);

        background: #c5c5c055;
        box-shadow: 0 0 8px 2px #aaaaaa99;

        box-sizing: border-box;
        border-right: 1px solid #999;
        border-top: 1px solid #999;
        border-bottom: 1px solid #999;
        border-top-right-radius: 25px;
        border-bottom-right-radius: 25px;

        overflow: hidden;
    }

    button#toggleButton {
        -webkit-appearance: none;
        appearance: none;

        width: 10px;
        height: 3rem;

        position: absolute;
        right: 5px;
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

    div#control-center-wrapper {
        width: calc(100% - 60px);
        min-width: 200px;
        max-height: calc(100% - 60px);

        position: absolute;
        right: 30px;
        top: 30px;
    }

</style>