#version 460 core

layout(binding = 0) uniform sampler2D ourTexture;

in vec3 ourColor;
in vec2 texCoord;

out vec4 FragColor;

void main() {
  // vec4 a = 
  FragColor = texture(ourTexture, texCoord);;

  // FragColor = vec4(1.0, 0.84, 0.2, 1.0);
}
