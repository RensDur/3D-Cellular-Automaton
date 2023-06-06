<script lang="ts">

    import P5 from 'p5-svelte';
    import { controller } from "$lib/stores/controller";

    let oldWindowWidth = 100;
    let oldWindowHeight = 100;

    export let windowWidth = 100;
    export let windowHeight = 100;


    //
    // Data
    //

    let graphRange = [-1, 5];
    let graphWidth = 0;
    let graphHeight = 0;

    let graphSpacing = 50;


    const sketch = (p5: any) => {
        p5.setup = () => {

            p5.createCanvas(windowWidth, windowHeight);

            p5.textAlign(p5.CENTER);

        }

        p5.draw = () => {

            // Resize the canvas if needed
            if (windowWidth != oldWindowWidth || windowHeight != oldWindowHeight) {
                oldWindowWidth = windowWidth;
                oldWindowHeight = windowHeight;
                p5.resizeCanvas(windowWidth, windowHeight);
            }

            // Continue drawing
            p5.clear();

            graphWidth = p5.width - 100 - graphSpacing;
            graphHeight = p5.height - 100 - graphSpacing;

            // Draw the grid of the graph
            let numberOfIterations = 10;

            if ($controller) {
                numberOfIterations = $controller.orderParameter.length;
            }

            // Vertical lines
            let numberOfLines = numberOfIterations % 50;
            let lineSpacing = graphWidth / numberOfLines;

            p5.fill(0);

            for (let i = 0; i <= numberOfLines; i++) {
                if (i == 0) {
                    p5.stroke(0);
                } else {
                    p5.stroke(150);
                }
                p5.line((p5.width - graphWidth - graphSpacing) + i*lineSpacing, graphSpacing, (p5.width - graphWidth - graphSpacing) + i*lineSpacing, graphHeight + graphSpacing);

                p5.text(String(p5.map(i, 0, numberOfLines, 0, numberOfIterations)), (p5.width - graphWidth - graphSpacing) + i*lineSpacing, graphHeight + graphSpacing + 20);
            }

            // Horizontal lines
            numberOfLines = (graphRange[1] - graphRange[0]) % 50;
            lineSpacing = graphHeight / numberOfLines;

            for (let i = 0; i <= numberOfLines; i++) {
                if (graphRange[0] + i == 0) {
                    p5.stroke(0);
                } else {
                    p5.stroke(150);
                }

                p5.line(p5.width - graphWidth - graphSpacing, graphHeight + graphSpacing - i*lineSpacing, p5.width - graphSpacing, graphHeight + graphSpacing - i*lineSpacing);

                p5.text(String(graphRange[0] + i), p5.width - graphWidth - graphSpacing - 20, graphHeight + graphSpacing - i*lineSpacing + 3);
            }


        }
    }





</script>


<div id="container">
    <P5 {sketch} />
</div>


<style>

    div#container {
        width: 100%;
        max-width: 100%;
        height: 100%;
        max-height: 100%;

        overflow: hidden;
    }

</style>