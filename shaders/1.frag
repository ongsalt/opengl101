#version 330 core

in vec4 vertexColor;
out vec4 FragColor;

void main() {
  // FragColor = vec4(1.0, 0.84, 0.2, 1.0);
  FragColor = vertexColor;
}
