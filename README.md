# Live Compositor components issues
To test those files, juste copy them into `./integration_tests/examples` folder.

## aliasing
While running `aliasing-issue` example, there is aliasing problem when the image is rescaled smaller.

## transition & rescaling on shader
While running `shader-issue` example, there is a loop transitioning between a fullscreen image and one scene with just a rescale of the image, and then the same transition but to a scene with a round-corner shader on top of it. It breaks image ratio and transition fluidness. 