#version 300 es

// Vertex position in object space coordinates
in vec3 vertexPosition;
// Surface normal at the vertex in object space coordinates
in vec3 vertexNormal;
// Texture coordinates at that vertex
in vec2 vertexTextureCoordinates;

// Model matrix
uniform mat4 mMatrix;
// View matrix
uniform mat4 vMatrix;
// Projection matrix
uniform mat4 pMatrix;

out vec3 normals;
out vec3 vertices;

// Main program for each vertex
void main() {
  vec4 vertexCamSpace = vMatrix * mMatrix * vec4(vertexPosition, 1.0);
  gl_Position = pMatrix * vertexCamSpace;
  normals = vertexNormal;
  vertices = vertexPosition;
}
