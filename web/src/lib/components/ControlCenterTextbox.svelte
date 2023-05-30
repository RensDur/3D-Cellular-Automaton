<script lang="ts">

    export let text = "";
    export let columnSpan = 1;

</script>


<div id="cstextbox" style="aspect-ratio: {columnSpan}/1;">
    <p>{text}</p>
</div>


<style>

    div#cstextbox {
        /* border: solid 1px #aaa;
        border-radius: 12px;

        box-shadow: 0 0 8px 2px #aaaaaa55; */

        background: #e5e5e055;
    }

    div#cstextbox p {
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

</style>