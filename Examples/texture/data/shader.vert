#version 330

layout(location = 0) in vec2 inVertexPosition;
layout(location = 1) in vec2 inTexureCoord;

out vec2 passTextureCoord;

void main() {
    gl_Position = vec4(inVertexPosition, 0.0, 1.0);
    passTextureCoord = inTexureCoord;
}