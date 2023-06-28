<script>

    import ControlCenterOverlay from "$lib/components/ControlCenterOverlay.svelte";
    import MainStage from "$lib/components/MainStage.svelte";
	import MainStageMarchingCubes from "$lib/components/MainStageMarchingCubes.svelte";
	import OrderParameterGraphOverlay from "$lib/components/OrderParameterGraphOverlay.svelte";

    let view_mode = 0;

</script>

<!-- Container that's allows no overlay of content or scrolling -->
<div id="container">
    <!-- Background that contains three.js instance -->
    <div id="background">
        {#if (view_mode == 0)}
        <MainStage />
        {:else}
        <MainStageMarchingCubes />
        {/if}
    </div>

    <!-- Foreground that contains controls and other overlays -->
    <div id="control-center">
        <ControlCenterOverlay/>
    </div>

    <div id="order-parameter">
        <OrderParameterGraphOverlay/>
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

    div#control-center {
        position: absolute;
        left: 0;
        top: 0;

        z-index: 1;
    }

    div#order-parameter {
        position: absolute;
        right: 0;
        top: 0;

        z-index: 1;
    }

</style>