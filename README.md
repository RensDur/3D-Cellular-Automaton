# Multi Species Turing Patterns in Three Dimensions

This repository contains the codebase that was developed as part of the [Research Project (2022/23)](https://github.com/TU-Delft-CSE/Research-Project) of [TU Delft](https://github.com/TU-Delft-CSE).

<hr>

**Author:** R.C.M. Dur <br>
**Date:** 28 June 2023

Delft University of Technology <br>
Under the supervision of M. Skrodzki and A.B.T. Barbaro.

<hr>

## Abstract
In 1952, Alan M. Turing presented a reaction-diffusion model that described formation of skin patterns. The patterns he predicted have later been found in various natural phenomena, such as in skins of fish or even in vegetation around termite hills. His patterns have even been taken to the micro-level. In 1984, David A. Young proposed a discretisation of this model, which enabled computer simulation. Both Turing and Young had only looked at two-dimensional patterns, until Martin Skrodzki and Konrad Polthier took the patterns to the third dimension in 2017. In this paper, these 3D simulations are generalised to produce patterns with more than two substances. We want to see whether Turing-like patterns also emerge there. To increase simulation speeds, a Graphics Processing Unit (GPU) implementation is described for this multi-species extension. Furthermore, we begin to investigate to what extent an order parameter can be defined, to analyse the formation of 3D structures. We found that our multi-species extension produced Turing-like structures that look similar to the ones found by former models. Our GPU simulation provided a significant performance increase. We also found that our order parameter can distinguish between well-mixed, well-segregated, and fully dominated states. However, it is yet unclear whether it can also be used to classify shapes.

_Read the full Bachelor thesis here: [Multi Species Turing Patterns in Three Dimensions](http://resolver.tudelft.nl/uuid:eecd5afe-de11-476c-b3e6-95fff175ca44)_

<hr>

# 3D Cellular Automaton

Since this thesis is concerned with both simulating and visualising various cellular automata in 3D, this application makes use of a simulation [server](./server/) and a [web client](./web/) for visualisation. The server is written in Rust and uses Apple's Metal API for GPU acceleration of the simulations. The web client is built using Svelte and is equipped with Three.js to visualise the cellular automata.

Because the server relies on Metal for its implementations, it can only be compiled and run on the macOS operating system. The web client can be run on virtually any device.

Both parts of the application are documented separately: [server](./server/), [web client](./web/).

<hr>

&copy; Rens Dur, 2023