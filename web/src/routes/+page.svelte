<script>

    import ControlCenterOverlay from "$lib/components/ControlCenterOverlay.svelte";
    import MainStage from "$lib/components/MainStage.svelte";
	import MainStageMarchingCubes from "$lib/components/MainStageMarchingCubes.svelte";

</script>

<!-- Container that's allows no overlay of content or scrolling -->
<div id="container">
    <!-- Background that contains three.js instance -->
    <div id="background">
        <MainStage/>
    </div>

    <!-- Foreground that contains controls and other overlays -->
    <div id="foreground">
        <ControlCenterOverlay/>
    </div>
</div>

<style>

    div#container {
        width: 100vw;
        height: 100vh;

        position: absolute;
        left: 0;
        top: 0;

        overflow: hidden;
    }

    div#background {
        width: 100%;
        height: 100%;

        position: absolute;
        left: 0;
        top: 0;

        z-index: 0;
    }

    div#foreground {
        position: absolute;
        left: 0;
        top: 0;

        z-index: 1;

        background: green;
    }

</style>