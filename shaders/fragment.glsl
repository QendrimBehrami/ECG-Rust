#version 300 es

// For better performance less precision
precision highp float;
out vec4 fragColor;

in vec3 normals;
in vec3 vertices;

// Main program for each fragment = pixel candidate
void main() {
  vec3 up = vec3(0.0,0.0,1.0);
  float angle = dot(normals,up);
  float scale = 1.5;
  fragColor = vec4(0.0,scale*angle,0.0,1.0);
}
