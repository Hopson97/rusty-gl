#version 330

out vec4 outColour;

in vec3 passVertexColour;

void main() {
    outColour = vec4(passVertexColour, 1.0);
}