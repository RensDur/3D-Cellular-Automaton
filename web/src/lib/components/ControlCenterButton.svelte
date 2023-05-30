<script lang="ts">

    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    export let text = "";
    export let columnSpan: number = 1;
    export let type = "standard";
    export let toggleAttach = -1;
    export let toggleAttachId = -1;
    
    let toggleActive = false;

    function toggleAttachUpdate(id: number) {
        toggleActive = id == toggleAttachId;
        dispatchStatus();
    }

    $: toggleAttachUpdate(toggleAttach);

    function toggle() {
        if (toggleAttach == -1) {
            // This button is not attached to another toggle button
            toggleActive = !toggleActive;
        } else {
            // This button is attached to another button, clicking on this button should always make this
            // button active
            toggleActive = true;
        }
        
        dispatchStatus();
    }

    function dispatchStatus() {
        if (toggleActive) {
            // The user just toggled this button, meaning our id should become selected
            toggleAttach = toggleAttachId;
        }

        dispatch(toggleActive ? "toggleOn" : "toggleOff");
    }

</script>

{#if type == "standard"}
<div id="csbutton" on:click={() => {dispatch("click");}} role="button" tabindex=0 style="aspect-ratio: {columnSpan}/1;">
    <p>{text}</p>
</div>
{:else if type == "toggle"}
<div id="csbutton" class={toggleActive ? "toggleActive" : "toggleNotActive"} on:click={toggle} role="button" tabindex=0 style="aspect-ratio: {columnSpan}/1;">
    <p>{text}</p>
</div>
{/if}

<style>

    div#csbutton {
        border: solid 1px #aaa;
        border-radius: 12px;

        box-shadow: 0 0 8px 2px #aaaaaa55;

        background: #e5e5e055;

        user-select: none;
    }

    div#csbutton:hover {
        background: #aaaaaa22;
        cursor: pointer;
    }

    div#csbutton p {
        width: 100%;

        margin: 0;
        padding: 0;

        position: relative;
        top: 50%;

        transform: translateY(-50%);
        -webkit-transform: translateY(-50%);

        font-family: Helvetica;
        font-weight: 500;
        font-size: 11pt;

        color: #555;

        text-align: center;
    }

    div#csbutton.toggleActive {
        background: #f5f5f555;
    }

    div#csbutton.toggleNotActive {
        background: #99999977;
    }

</style>