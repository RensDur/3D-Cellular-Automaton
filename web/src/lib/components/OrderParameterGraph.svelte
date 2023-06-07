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

    let graphRange = [-1, 1];
    let graphWidth = 0;
    let graphHeight = 0;

    let graphSpacing = 35;


    const sketch = (p5: any) => {
        p5.setup = () => {

            p5.createCanvas(windowWidth, windowHeight);

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

            graphWidth = p5.width - 80 - graphSpacing;
            graphHeight = p5.height - 70 - graphSpacing;

            // Draw the grid of the graph
            let numberOfIterations = 10;

            if ($controller) {
                numberOfIterations = Math.max($controller.orderParameter.length, 10);
            }

            // Vertical lines
            let numberOfLines = Math.max(numberOfIterations % 20, 6);
            let lineSpacing = graphWidth / numberOfLines;

            p5.fill(0);
            p5.strokeWeight(1);

            p5.textAlign(p5.CENTER);
            p5.textSize(12);

            for (let i = 0; i <= numberOfLines; i++) {
                if (i == 0) {
                    p5.stroke(0);
                } else {
                    p5.stroke(150);
                }
                p5.line((p5.width - graphWidth - graphSpacing) + i*lineSpacing, graphSpacing, (p5.width - graphWidth - graphSpacing) + i*lineSpacing, graphHeight + graphSpacing);

                p5.text(String(p5.map(i, 0, numberOfLines, 0, numberOfIterations).toFixed(2)), (p5.width - graphWidth - graphSpacing) + i*lineSpacing, graphHeight + graphSpacing + 20);
            }

            // Horizontal lines
            numberOfLines = graphRange[1] - graphRange[0];
            lineSpacing = graphHeight / numberOfLines;

            let yZeroStart = 0;

            for (let i = 0; i <= numberOfLines; i++) {
                if (graphRange[0] + i == 0) {
                    p5.stroke(0);

                    // Store this offset for later use
                    yZeroStart = i;
                } else {
                    p5.stroke(150);
                }

                p5.line(p5.width - graphWidth - graphSpacing, graphHeight + graphSpacing - i*lineSpacing, p5.width - graphSpacing, graphHeight + graphSpacing - i*lineSpacing);

                p5.text(String(p5.map(i, 0, numberOfLines, graphRange[0], graphRange[1])), p5.width - graphWidth - graphSpacing - 20, graphHeight + graphSpacing - i*lineSpacing + 3);

                // Also draw some lines between the whole numbers
                if (i < numberOfLines) {
                    let numOfSeparationLines = 10;
                    let separation = lineSpacing / numOfSeparationLines;
                    p5.stroke(220);
                    for (let j = 1; j < numOfSeparationLines; j++) {
                        p5.line(p5.width - graphWidth - graphSpacing, graphHeight + graphSpacing - i*lineSpacing - j*separation, p5.width - graphSpacing, graphHeight + graphSpacing - i*lineSpacing - j*separation);

                        if (j % 2 == 0) {
                            p5.text(String(p5.map(i + j/numOfSeparationLines, 0, numberOfLines, graphRange[0], graphRange[1]).toFixed(2)), p5.width - graphWidth - graphSpacing - 20, graphHeight + graphSpacing - i*lineSpacing - j*separation + 3);
                        }
                    }
                }

            }

            // Display the graph
            p5.stroke(56, 182, 255);
            p5.strokeWeight(5);
            p5.noFill();

            p5.beginShape();

            if ($controller) {

                let dxPerPoint = graphWidth / Math.max($controller.orderParameter.length, 10);
                let dyPerPoint = graphHeight / (graphRange[1] - graphRange[0]);

                for (let i = 0; i < $controller.orderParameter.length; i++) {

                    let yVal = $controller.orderParameter[i];

                    // If this yvalue falls out of the bounds, end the shape and start a new one
                    if (yVal < graphRange[0] || yVal > graphRange[1]) {
                        p5.endShape();
                        p5.beginShape();
                    } else {

                        // Else, add this point to the graph
                        p5.vertex(p5.width - graphWidth - graphSpacing + i*dxPerPoint, graphHeight + graphSpacing - yZeroStart*dyPerPoint - yVal*dyPerPoint);

                    }

                }
            }

            p5.endShape();




            // Print the necessary graph properties
            p5.stroke(0);
            p5.strokeWeight(1);
            p5.fill(0);

            p5.textAlign(p5.RIGHT);
            p5.textSize(15);
            p5.text("Number of iterations", p5.width - graphSpacing - 20, graphSpacing + graphHeight + 60);

            p5.translate(p5.width - graphSpacing - graphWidth - 60, graphSpacing + 20);
            p5.rotate(- p5.PI / 2);

            p5.text("Order Parameter", 0, 0);

            p5.rotate(p5.PI / 2);
            p5.translate(- (p5.width - graphSpacing - graphWidth - 60), - (graphSpacing + 20));

            p5.textAlign(p5.CENTER);
            p5.textSize(22);

            p5.text("Order Parameter per iteration", p5.width - graphSpacing - graphWidth/2, graphSpacing / 2);
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