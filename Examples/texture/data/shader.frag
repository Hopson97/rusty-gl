#version 330

out vec4 outColour;

in vec3 passVertexColour;
in vec2 passTextureCoord;

uniform sampler2D textureSampler;

void main() {
    outColour = texture(textureSampler, passTextureCoord) * vec4(passVertexColour, 1.0);
}