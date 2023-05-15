<script lang="ts">

    export let title: String = "";
    export let columns: number = 4;

</script>

<p id="title">{title}</p>
<div id="group" style="grid-template-columns: repeat({columns}, minmax(0, 1fr));">
    <slot></slot>
</div>

<style>

    p#title {
        font-family: Helvetica;
        font-weight: 700;
        font-size: 11pt;

        color: #aaa;

        margin: 0;
        padding: 0;

        position: relative;
        left: 3px;
    }

    div#group {
        width: calc(100% - 24px);
        min-width: 100px;

        padding: 12px;

        margin-top: 2px;
        margin-bottom: 30px;

        display: grid;
        gap: 12px;

        border-radius: 15px;
        border: solid 1px #aaa;

        box-shadow: inset 0 0 8px 2px #aaaaaa55;

        background: transparent;

        overflow: hidden;
    }

</style>