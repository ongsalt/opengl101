#version 460 core

in vec4 vertexColor;
in vec4 vertexPosition;
out vec4 FragColor;

void main() {
  // FragColor = vec4(1.0, 0.84, 0.2, 1.0);
  // FragColor = vertexColor;
  FragColor = vertexPosition;
}
