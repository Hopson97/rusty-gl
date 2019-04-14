#version 330

layout(location = 0) in vec2 inVertexPosition;
layout(location = 1) in vec3 inVertexColour;

out vec3 passVertexColour;

void main() {
    gl_Position = vec4(inVertexPosition, 0.0, 1.0);
    passVertexColour = inVertexColour;
}